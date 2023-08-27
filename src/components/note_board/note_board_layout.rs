use crate::types::Key;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NoteBoardLayout {
    MajorKeys,
    MinorKeys,
    NaturalNotes,
}

impl From<Key> for NoteBoardLayout {
    fn from(value: Key) -> Self {
        match value {
            Key::Major(_) => Self::MajorKeys,
            Key::Minor(_) => Self::MinorKeys,
        }
    }
}
