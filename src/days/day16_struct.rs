use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
pub struct Field {
    pub name: String,
    pub range1: RangeInclusive<u32>,
    pub range2: RangeInclusive<u32>
}

impl Field {
    pub fn is_number_allowed(&self, number: u32) -> bool {
        self.range1.contains(&number) || self.range2.contains(&number)
    }
}