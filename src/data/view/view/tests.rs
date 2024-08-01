use super::super::super::{
    name::Name,
    node::node::{
        DocumentNode, AnchorRequestNode, ListNode, MapNode, MarkedNode, Node, TaggedNode, AnchorCreationNode,
    },
};
use super::*;
use std::{collections::HashMap, path::PathBuf};

fn test_data() -> Data {
    let name = |i: &str| Name::new(i.into()).unwrap();
    Data::new([
        MarkedNode::new(Node::Null, Mark { line: 2, symbol: 5 }),
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
                path: PathBuf::from("dir/name.ieml"),
                anchors: Default::default(),
                document_anchors: Default::default(),
                parent: None,
            }),
            Default::default(),
        ),
        MarkedNode::new(
            Node::AnchorCreation(AnchorCreationNode::new(name("anchor"), 4)),
            Default::default(),
        ),
        MarkedNode::new(
            Node::AnchorRequest(AnchorRequestNode::new(name("anchor"), 4)),
            Default::default(),
        ),
    ])
}

fn make_another_type_error(
    node_type: NodeType,
    requested_type: NodeType,
    mark: Mark,
) -> marked::AnotherTypeError {
    marked::AnotherTypeError::new(mark, AnotherTypeError::new(requested_type, node_type))
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
    assert!(!view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(NodeType::Null, NodeType::Raw, mark))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::List,
            mark
        ))
    );
    assert_eq!(
        view.map(),
        Err(make_another_type_error(NodeType::Null, NodeType::Map, mark))
    );
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::Document,
            mark
        ))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::AnchorCreation,
            mark
        ))
    );
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::AnchorCreation,
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
    assert!(!view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(view.raw().unwrap().raw(), "hello");
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(NodeType::Raw, NodeType::List, mark))
    );
    assert_eq!(
        view.map(),
        Err(make_another_type_error(NodeType::Raw, NodeType::Map, mark))
    );
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(NodeType::Raw, NodeType::Document, mark))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::AnchorCreation,
            mark
        ))
    );
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::AnchorCreation,
            mark
        ))
    );
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
    assert!(!view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(view.string().unwrap().string(), "hello");
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::List,
            mark
        ))
    );
    assert_eq!(
        view.map(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Map,
            mark
        ))
    );
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Document,
            mark
        ))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::AnchorCreation,
            mark
        ))
    );
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::AnchorCreation,
            mark
        ))
    );
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
    assert!(!view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(NodeType::List, NodeType::Raw, mark))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::String,
            mark
        ))
    );
    if let Node::List(node) = &data.get(3).node {
        assert_eq!(
            view.list(),
            Ok(ListView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a list");
    }
    assert_eq!(
        view.map(),
        Err(make_another_type_error(NodeType::List, NodeType::Map, mark))
    );
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::Document,
            mark
        ))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::AnchorCreation,
            mark
        ))
    );
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::AnchorCreation,
            mark
        ))
    );
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
    assert!(!view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(NodeType::Map, NodeType::List, mark))
    );
    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(NodeType::Map, NodeType::Document, mark))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::AnchorCreation,
            mark
        ))
    );
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::AnchorCreation,
            mark
        ))
    );
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
    assert!(view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::List,
            mark
        ))
    );
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
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::Document,
            mark
        ))
    );
    if let Node::AnchorCreation(node) = &data.get(7).node {
        assert_eq!(
            view.anchor_creation(),
            Ok(AnchorCreationView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(view.anchor_name().unwrap().as_str(), "anchor");
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
    assert!(view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(NodeType::Document, NodeType::Raw, mark))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::Document,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::Document,
            NodeType::List,
            mark
        ))
    );
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
    if let Node::AnchorCreation(node) = &data.get(7).node {
        assert_eq!(
            view.anchor_creation(),
            Ok(AnchorCreationView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::Document,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(view.anchor_name().unwrap().as_str(), "anchor");
}

#[test]
fn test_anchor_creation() {
    let data = test_data();
    let view = View::new(data.get(7), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::AnchorCreation);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(view.is_anchor_creation());
    assert!(!view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::List,
            mark
        ))
    );
    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::Document,
            mark
        ))
    );
    if let Node::AnchorCreation(node) = &data.get(7).node {
        assert_eq!(
            view.anchor_creation(),
            Ok(AnchorCreationView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.anchor_request(),
        Err(make_another_type_error(
            NodeType::AnchorCreation,
            NodeType::AnchorRequest,
            mark
        ))
    );
    assert_eq!(view.anchor_name().unwrap().as_str(), "anchor");
}

#[test]
fn test_anchor_request() {
    let data = test_data();
    let view = View::new(data.get(8), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::AnchorRequest);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_document());
    assert!(!view.is_anchor_creation());
    assert!(view.is_anchor_request());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::List,
            mark
        ))
    );
    if let Node::Map(node) = &data.get(4).node {
        assert_eq!(
            view.map(),
            Ok(MapView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a map");
    }
    assert_eq!(
        view.tagged(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.document(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::Document,
            mark
        ))
    );
    assert_eq!(
        view.anchor_creation(),
        Err(make_another_type_error(
            NodeType::AnchorRequest,
            NodeType::AnchorCreation,
            mark
        ))
    );
    if let Node::AnchorRequest(node) = &data.get(8).node {
        assert_eq!(
            view.anchor_request(),
            Ok(AnchorRequestView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a get anchor");
    }
    assert_eq!(view.anchor_name().unwrap().as_str(), "anchor");
}
