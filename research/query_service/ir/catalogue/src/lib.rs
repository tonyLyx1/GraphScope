pub mod codec;
pub mod extend_step;
pub mod pattern;
pub mod pattern_meta;

type ID = i32;
type LabelID = i32;
type Index = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Out = 0,
    In,
}