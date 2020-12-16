use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
pub struct Field {
    pub name: String,
    pub range1: RangeInclusive<u32>,
    pub range2: RangeInclusive<u32>
}