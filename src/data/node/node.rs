use super::super::mark::Mark;
pub(crate) use super::{
    document_node::DocumentNode, anchor_request_node::AnchorRequestNode, list_node::ListNode, map_node::MapNode,
    tag_node::TaggedNode, anchor_creation_node::AnchorCreationNode,
};

pub(crate) type RawNode = String;
pub(crate) type StringNode = String;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum Node {
    #[default]
    Null,
    Raw(RawNode),
    String(StringNode),
    List(ListNode),
    Map(MapNode),
    Tagged(TaggedNode),
    Document(DocumentNode),
    AnchorCreation(AnchorCreationNode),
    AnchorRequest(AnchorRequestNode),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MarkedNode {
    pub node: Node,
    pub mark: Mark,
}

impl MarkedNode {
    pub fn new(node: Node, mark: Mark) -> Self {
        Self { node, mark }
    }
}
