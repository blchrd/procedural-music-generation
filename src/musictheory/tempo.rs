#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tempo(pub u16);

impl From<Tempo> for u16 {
    fn from(t: Tempo) -> Self {
        t.0
    }
}

impl From<u16> for Tempo {
    fn from(i: u16) -> Self {
        Self(i)
    }
}

impl Tempo {
    pub fn get_bps(self) -> f32 {
        self.0 as f32 / 60.0
    }
}