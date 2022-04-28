pub mod codec;
pub mod extend_step;
pub mod pattern;
pub mod pattern_meta;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Out = 0,
    In,
}