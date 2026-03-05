#[derive(Clone, Copy, PartialEq)]
pub enum ClockFormat {
    H12,
    H24,
}

impl ClockFormat {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::H12 => Self::H24,
            Self::H24 => Self::H12,
        };
    }

    pub fn is_12h(self) -> bool {
        matches!(self, Self::H12)
    }

    pub fn from_u8(value: u8) -> Self {
        if value == 12 { Self::H12 } else { Self::H24 }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::H12 => 12,
            Self::H24 => 24,
        }
    }
}
