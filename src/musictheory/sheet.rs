use core::fmt;

use crate::musictheory::measure::Measure;

#[derive(Debug, Clone)]
pub struct Sheet {
    pub measures: Vec<Measure>
}

impl Sheet {
    pub fn new() -> Self {
        Sheet {
            measures: Vec::<Measure>::new()
        }
    }

    pub fn add_measure(&mut self, measure: Measure) {
        self.measures.push(measure);
    }
}

impl fmt::Display for Sheet {
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