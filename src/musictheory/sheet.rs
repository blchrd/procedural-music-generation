use core::fmt;

use super::pattern::Pattern;

#[derive(Debug, Clone)]
pub struct Sheet {
    pub patterns: Vec<Pattern>
}

impl Sheet {
    pub fn new() -> Self {
        Sheet {
            patterns: Vec::<Pattern>::new()
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        self.patterns.iter().for_each(|m| {
            ret.push_str(&format!("{} \n", *m))
        });

        write!(
            f,
            "{}",
            ret
        )
    }
}