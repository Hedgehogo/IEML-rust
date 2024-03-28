#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
    Null = 0,
    Raw,
    String,
    List,
    Map,
    Tagged,
    File,
    TakeAnchor,
    GetAnchor,
}
