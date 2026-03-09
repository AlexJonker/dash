use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;

use eframe::egui;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use rand::seq::SliceRandom;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

#[derive(Clone)]
pub struct Track {
    pub path: PathBuf,
    pub artist: String,
    pub album: String,
    pub title: String,
    pub duration: Duration,
}

pub struct MusicSession {
    tracks: Vec<Track>,
    queue: Vec<usize>,
    queue_pos: usize,
    device_sink: Option<MixerDeviceSink>,
    player: Option<Player>,
    paused: bool,
    last_error: Option<String>,
    cover_cache: HashMap<usize, Option<egui::TextureHandle>>,
    volume: f32,
    shuffle: bool,
    loop_track: bool,
}

impl MusicSession {
    pub fn new(music_folder: &str, volume: f32, shuffle: bool, loop_track: bool) -> Self {
        let folder = PathBuf::from(music_folder);

        let tracks = scan_music_library(&folder);
        let (device_sink, player, error) = match DeviceSinkBuilder::open_default_sink() {
            Ok(sink) => {
                let player = Player::connect_new(sink.mixer());
                (Some(sink), Some(player), None)
            }
            Err(err) => (None, None, Some(format!("Audio output unavailable: {err}"))),
        };

        let mut session: MusicSession = Self {
            tracks,
            queue: Vec::new(),
            queue_pos: 0,
            device_sink,
            player,
            paused: true,
            last_error: error,
            cover_cache: HashMap::new(),
            volume: volume.clamp(0.0, 1.0),
            shuffle,
            loop_track,
        };

        session.apply_volume();
        session.ensure_queue_initialized();
        session
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.current_track_index()
            .and_then(|idx| self.tracks.get(idx))
    }

    pub fn current_cover(&mut self, ctx: &egui::Context) -> Option<&egui::TextureHandle> {
        let idx = self.current_track_index()?;

        if !self.cover_cache.contains_key(&idx) {
            let texture = self
                .tracks
                .get(idx)
                .and_then(|track| extract_cover_texture(ctx, &track.path));
            self.cover_cache.insert(idx, texture);
        }

        self.cover_cache
            .get(&idx)
            .and_then(|texture| texture.as_ref())
    }

    pub fn is_playing(&self) -> bool {
        self.player
            .as_ref()
            .map(|player| !player.is_paused() && !player.empty())
            .unwrap_or(false)
    }

    pub fn current_position_secs(&self) -> f32 {
        self.player
            .as_ref()
            .map(|player| player.get_pos().as_secs_f32())
            .unwrap_or(0.0)
    }

    pub fn current_duration_secs(&self) -> Option<f32> {
        let secs = self.current_track()?.duration.as_secs_f32();
        if secs > 0.0 { Some(secs) } else { None }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.apply_volume();
    }

    pub fn is_loop_enabled(&self) -> bool {
        self.loop_track
    }

    pub fn loop_toggle(&mut self) {
        self.loop_track = !self.loop_track;
    }

    pub fn is_shuffle_enabled(&self) -> bool {
        self.shuffle
    }

    pub fn shuffle_toggle(&mut self) {
        self.shuffle = !self.shuffle;

        if self.tracks.is_empty() {
            return;
        }

        let current = self.current_track_index();
        if self.queue.is_empty() {
            self.queue = (0..self.tracks.len()).collect();
        }

        self.rebuild_queue(current);
    }

    pub fn seek_to_secs(&mut self, secs: f32) {
        let Some(player) = &self.player else {
            return;
        };

        let Some(duration) = self.current_track().map(|track| track.duration) else {
            return;
        };

        let target_secs = secs.clamp(0.0, duration.as_secs_f32());
        let target = Duration::from_secs_f32(target_secs);

        if let Err(err) = player.try_seek(target) {
            self.last_error = Some(format!("Failed to seek: {err}"));
        }
    }

    pub fn play_pause_toggle(&mut self) {
        if !self.ensure_queue_initialized() {
            return;
        }

        if self.player.is_none() {
            self.play_current();
            return;
        }

        if let Some(sink) = &self.player {
            if sink.empty() {
                self.play_current();
                return;
            }

            if self.paused {
                sink.play();
                self.paused = false;
            } else {
                sink.pause();
                self.paused = true;
            }
        }
    }

    pub fn next(&mut self) {
        if !self.ensure_queue_initialized() {
            return;
        }

        self.queue_pos = (self.queue_pos + 1) % self.queue.len();
        self.play_current();
    }

    pub fn previous(&mut self) {
        if !self.ensure_queue_initialized() {
            return;
        }

        self.queue_pos = if self.queue_pos == 0 {
            self.queue.len() - 1
        } else {
            self.queue_pos - 1
        };
        self.play_current();
    }

    pub fn tick(&mut self) {
        let finished = self
            .player
            .as_ref()
            .map(|sink| sink.empty() && !self.paused)
            .unwrap_or(false);

        if finished {
            if self.loop_track {
                // If loop is enabled then just play the track again instead of going to the next one.
                self.play_current();
            } else {
                // If loop is not enabled then play th enext track.
                self.next();
            }
        }
    }

    fn current_track_index(&self) -> Option<usize> {
        self.queue.get(self.queue_pos).copied()
    }

    fn ensure_queue_initialized(&mut self) -> bool {
        if self.queue.is_empty() && !self.tracks.is_empty() {
            self.rebuild_queue(None);
        }

        !self.queue.is_empty()
    }

    fn rebuild_queue(&mut self, current_first: Option<usize>) {
        if self.tracks.is_empty() {
            self.queue.clear();
            self.queue_pos = 0;
            return;
        }

        self.queue = (0..self.tracks.len()).collect();

        if self.shuffle {
            let mut rng = rand::rng();
            self.queue.shuffle(&mut rng);

            if let Some(current) = current_first {
                if let Some(pos) = self.queue.iter().position(|&idx| idx == current) {
                    self.queue.swap(0, pos);
                }
            }

            self.queue_pos = 0;
        } else if let Some(current) = current_first {
            self.queue_pos = self
                .queue
                .iter()
                .position(|&idx| idx == current)
                .unwrap_or(0);
        } else {
            self.queue_pos = 0;
        }
    }

    fn play_current(&mut self) {
        let Some(track_idx) = self.current_track_index() else {
            return;
        };

        let Some(player) = &self.player else {
            self.last_error = Some("No audio output available".to_string());
            return;
        };

        let track = &self.tracks[track_idx];

        let file = match File::open(&track.path) {
            Ok(file) => file,
            Err(err) => {
                self.last_error = Some(format!("Failed to open file: {err}"));
                return;
            }
        };

        let decoder = match Decoder::try_from(file) {
            Ok(decoder) => decoder,
            Err(err) => {
                self.last_error = Some(format!("Failed to decode audio: {err}"));
                return;
            }
        };

        player.clear();
        player.append(decoder);
        player.set_volume(self.volume);
        player.play();

        self.paused = false;
        self.last_error = None;

        let _ = self.device_sink.as_ref();
    }

    fn apply_volume(&mut self) {
        if let Some(player) = &self.player {
            player.set_volume(self.volume);
        }
    }
}

fn scan_music_library(folder: &Path) -> Vec<Track> {
    let mut tracks = Vec::new();

    fn visit_dir(folder: &Path, dir: &Path, out: &mut Vec<Track>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dir(folder, &path, out);
                continue;
            }

            if !is_supported_audio(&path) {
                continue;
            }

            let relative = path.strip_prefix(folder).unwrap_or(&path);
            let mut comps = relative.components();
            let artist = comps
                .next()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown Artist".to_string());
            let album = comps
                .next()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown Album".to_string());

            let title = path
                .file_stem()
                .map(|name| name.to_string_lossy().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown Track".to_string());

            let duration = Probe::open(&path)
                .ok()
                .and_then(|probe| probe.read().ok())
                .map(|tagged| tagged.properties().duration())
                .unwrap_or(Duration::ZERO);

            out.push(Track {
                path,
                artist,
                album,
                title,
                duration,
            });
        }
    }

    visit_dir(folder, folder, &mut tracks);
    tracks
}

fn is_supported_audio(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    matches!(
        ext.as_deref(),
        Some("mp3") | Some("flac") | Some("m4a") | Some("aac") | Some("wav") | Some("ogg")
    )
}

fn extract_cover_texture(ctx: &egui::Context, path: &Path) -> Option<egui::TextureHandle> {
    let tagged = Probe::open(path).ok()?.read().ok()?;
    let tag = tagged.primary_tag().or_else(|| tagged.first_tag())?;
    let pic = tag.pictures().first()?;
    let image = image::load_from_memory(pic.data()).ok()?.to_rgba8();

    let width = usize::try_from(image.width()).ok()?;
    let height = usize::try_from(image.height()).ok()?;
    let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], image.as_raw());

    Some(ctx.load_texture(
        format!("cover-{}", path.display()),
        color_image,
        egui::TextureOptions::LINEAR,
    ))
}
