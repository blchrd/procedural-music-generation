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
    pub fn get_bpm(self) -> f64 {
        self.0 as f64 / 60.0
    }
}