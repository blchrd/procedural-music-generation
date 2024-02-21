// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Semitone(pub i8);

impl From<i8> for Semitone {
    fn from(i: i8) -> Self {
        Self(i)
    }
}

impl From<Semitone> for i8 {
    fn from(s: Semitone) -> Self {
        s.0
    }
}