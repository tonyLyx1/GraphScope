use crate::structure::AsTag;

pub enum SelectKey {
    Current,
    Tagged(Vec<AsTag>),
}

pub enum ByModulating {
    Itself,
    Keys,
    Values,
    Id,
    Label,
    Property(String),
    Properties(Vec<String>),
    ValueMap(Vec<String>),
    Prepared,
}
