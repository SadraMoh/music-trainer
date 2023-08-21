pub enum Key {
    Major(MajorKey),
    Minor(MinorKey),
}

#[derive(Clone, Copy, PartialEq)]
pub enum MajorKey {
    F,
    C,
    G,
    D,
    A,
    E,
    B,
    CFlat,
    FSharp,
    GFlat,
    DFlat,
    CSharp,
    AFlat,
    EFlat,
    BFlat,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MinorKey {
    A,
    E,
    B,
    FSharp,
    CSharp,
    GSharp,
    AFlat,
    DSharp,
    EFlat,
    BFlat,
    ASharp,
    F,
    C,
    G,
    D,
}
