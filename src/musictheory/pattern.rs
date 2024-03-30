use core::fmt;

use super::measure::Measure;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub measures: Vec<Measure>,
}

impl Pattern {
    pub fn new(name: String) -> Self {
        Pattern {
            name,
            measures: Vec::<Measure>::new(),
        }
    }

    pub fn add_measure(&mut self, measure: Measure) {
        self.measures.push(measure);
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        self.measures.iter().for_each(|m| {
            ret.push_str(&format!("{} \n", *m))
        });

        write!(
            f,
            "{}",
            ret
        )
    }
}