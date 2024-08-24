use super::super::super::{
    name::Name,
    node::node::{AnchorNode, DocumentNode, ListNode, MapNode, MarkedNode, Node, TaggedNode},
};
use super::*;
use std::collections::HashMap;

fn test_data() -> Data {
    let name = |i: &str| Name::new(i.into()).unwrap();
    Data::new([
        MarkedNode::new(Node::Null, Mark::new(2, 5)),
        MarkedNode::new(Node::Raw("hello".into()), Default::default()),
        MarkedNode::new(Node::String("hello".into()), Default::default()),
        MarkedNode::new(Node::List(ListNode::new(vec![0, 1])), Default::default()),
        MarkedNode::new(
            Node::Map(MapNode::new(HashMap::from([
                (name("first"), 2),
                (name("second"), 3),
                (name("third"), 8),
            ]))),
            Default::default(),
        ),
        MarkedNode::new(
            Node::Tagged(TaggedNode::new(name("tag"), 7)),
            Default::default(),
        ),
        MarkedNode::new(
            Node::Document(DocumentNode {
                node_index: 5,
                path: "dir/name.ieml".into(),
                anchors: Default::default(),
                document_anchors: Default::default(),
                parent: None,
            }),
            Default::default(),
        ),
        MarkedNode::new(
            Node::Anchor(AnchorNode::new(name("anchor"), 4, true)),
            Default::default(),
        ),
        MarkedNode::new(
            Node::Anchor(AnchorNode::new(name("anchor"), 4, false)),
            Default::default(),
        ),
    ])
}

fn make_invalid_type_error(
    unexpected_type: NodeType,
    expected_types: &'static [NodeType],
    mark: Mark,
) -> marked::InvalidTypeError {
    marked::InvalidTypeError::new(mark, InvalidTypeError::new(unexpected_type, expected_types))
}

#[test]
fn test_null() {
    let data = test_data();
    let view = View::new(data.get(0), &data, ());
    let mark = Mark { line: 2, symbol: 5 };

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Null);

    assert!(view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(!view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor());

    assert_eq!(
        view.raw(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[
                NodeType::Raw,
                NodeType::Tagged,
                NodeType::Anchor,
                NodeType::Document,
            ],
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[
                NodeType::String,
                NodeType::Tagged,
                NodeType::Anchor,
                NodeType::Document,
            ],
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[
                NodeType::List,
                NodeType::Tagged,
                NodeType::Anchor,
                NodeType::Document,
            ],
            mark
        ))
    );
    assert_eq!(
        view.map(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[
                NodeType::Map,
                NodeType::Tagged,
                NodeType::Anchor,
                NodeType::Document,
            ],
            mark
        ))
    );
    assert_eq!(
        view.tagged(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[NodeType::Tagged, NodeType::Anchor, NodeType::Document],
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[NodeType::Document, NodeType::Tagged, NodeType::Anchor],
            mark
        ))
    );
    assert_eq!(
        view.anchor(),
        Err(make_invalid_type_error(
            NodeType::Null,
            &[NodeType::Anchor, NodeType::Tagged, NodeType::Document],
            mark
        ))
    );
}

#[test]
fn test_raw() {
    let data = test_data();
    let view = View::new(data.get(1), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Raw);

    assert!(!view.is_null());
    assert!(view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(!view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor());

    assert_eq!(view.raw().unwrap().raw(), "hello");
    assert!(view.string().is_err());
    assert!(view.list().is_err());
    assert!(view.map().is_err());
    assert!(view.tagged().is_err());
    assert!(view.document().is_err());
    assert!(view.anchor().is_err());
}

#[test]
fn test_string() {
    let data = test_data();
    let view = View::new(data.get(2), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::String);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(view.is_string());
    assert!(!view.is_list());
    assert!(!view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor());

    assert!(view.raw().is_err());
    assert_eq!(view.string().unwrap().string(), "hello");
    assert!(view.list().is_err());
    assert!(view.map().is_err());
    assert!(view.tagged().is_err());
    assert!(view.document().is_err());
    assert!(view.anchor().is_err());
}

#[test]
fn test_list() {
    let data = test_data();
    let view = View::new(data.get(3), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::List);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(view.is_list());
    assert!(!view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());

    if let Node::List(node) = &data.get(3).node {
        assert_eq!(
            view.list(),
            Ok(ListView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a list");
    }

    assert!(view.map().is_err());
    assert!(view.tagged().is_err());
    assert!(view.document().is_err());
    assert!(view.anchor().is_err());
}

#[test]
fn test_map() {
    let data = test_data();
    let view = View::new(data.get(4), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Map);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());
    assert!(view.list().is_err());

    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }

    assert!(view.tagged().is_err());
    assert!(view.document().is_err());
    assert!(view.anchor().is_err());
}

#[test]
fn test_tagged() {
    let data = test_data();
    let view = View::new(data.get(5), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Tagged);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(view.is_tagged());
    assert!(!view.is_document());
    assert!(view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());
    assert!(view.list().is_err());

    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }

    if let Node::Tagged(node) = &data.get(5).node {
        assert_eq!(
            view.tagged(),
            Ok(TaggedView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a tagged");
    }

    assert!(view.document().is_err());

    if let Node::Anchor(node) = &data.get(7).node {
        assert_eq!(
            view.anchor(),
            Ok(AnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
}

#[test]
fn test_document() {
    let data = test_data();
    let view = View::new(data.get(6), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Document);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(view.is_tagged());
    assert!(view.is_document());
    assert!(view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());
    assert!(view.list().is_err());

    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }
    if let Node::Tagged(node) = &data.get(5).node {
        assert_eq!(
            view.tagged(),
            Ok(TaggedView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a tagged");
    }
    if let Node::Document(node) = &data.get(6).node {
        assert_eq!(
            view.document(),
            Ok(DocumentView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    if let Node::Anchor(node) = &data.get(7).node {
        assert_eq!(
            view.anchor(),
            Ok(AnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
}

#[test]
fn test_anchor_creation() {
    let data = test_data();
    let view = View::new(data.get(7), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Anchor);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());
    assert!(view.list().is_err());

    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }

    assert!(view.tagged().is_err());
    assert!(view.document().is_err());

    if let Node::Anchor(node) = &data.get(7).node {
        assert_eq!(
            view.anchor(),
            Ok(AnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
}

#[test]
fn test_anchor_request() {
    let data = test_data();
    let view = View::new(data.get(8), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::Anchor);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(view.is_anchor());

    assert!(view.raw().is_err());
    assert!(view.string().is_err());
    assert!(view.list().is_err());

    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }

    assert!(view.tagged().is_err());
    assert!(view.document().is_err());

    if let Node::Anchor(node) = &data.get(8).node {
        assert_eq!(
            view.anchor(),
            Ok(AnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a get anchor");
    }
}
