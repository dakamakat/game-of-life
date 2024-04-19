use fixedbitset::FixedBitSet;

use crate::utilities::bool_random;

#[derive(Debug)]
pub enum Patterns {
    None(NonePattern),
    Random(RandomPattern),
}

pub fn init_pattern(pattern: u32, cells: &mut FixedBitSet, size: usize) {
    match pattern {
        0 => NonePattern::init(cells, size),
        1 => RandomPattern::init(cells, size),
        _ => unreachable!()
    };
}

#[derive(Debug)]
pub struct NonePattern {}

#[derive(Debug)]
pub struct RandomPattern {}

impl Pattern for NonePattern {
    fn init(_cells: &mut FixedBitSet,_sizee: usize) {}
}

impl Pattern for RandomPattern {
    fn init(cells: &mut FixedBitSet, size: usize) {
        for i in 0..size {
            cells.set(i, bool_random());
        }
    }
}

trait Pattern {
    fn init(_cells: &mut FixedBitSet, _size: usize) {}
}
