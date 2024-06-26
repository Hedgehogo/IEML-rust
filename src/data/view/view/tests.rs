use super::super::super::node::node::{
    FileNode, GetAnchorNode, ListNode, MapNode, MarkedNode, Node, TaggedNode, TakeAnchorNode,
};
use super::*;
use std::{collections::HashMap, path::PathBuf};

fn test_data() -> Data {
    Data::new([
        MarkedNode::new(Node::Null, Mark { line: 2, symbol: 5 }),
        MarkedNode::new(Node::Raw("hello".into()), Default::default()),
        MarkedNode::new(Node::String("hello".into()), Default::default()),
        MarkedNode::new(Node::List(ListNode::new(vec![0, 1])), Default::default()),
        MarkedNode::new(
            Node::Map(MapNode::new(HashMap::from([
                ("first".to_string(), 2),
                ("second".to_string(), 3),
                ("third".to_string(), 8),
            ]))),
            Default::default(),
        ),
        MarkedNode::new(
            Node::Tagged(TaggedNode::new("tag".into(), 7)),
            Default::default(),
        ),
        MarkedNode::new(
            Node::File(FileNode {
                node_index: 5,
                path: PathBuf::from("dir/name.ieml"),
                anchors: Default::default(),
                file_anchors: Default::default(),
                parent: None,
            }),
            Default::default(),
        ),
        MarkedNode::new(
            Node::TakeAnchor(TakeAnchorNode::new("anchor".into(), 4)),
            Default::default(),
        ),
        MarkedNode::new(
            Node::GetAnchor(GetAnchorNode::new("anchor".into(), 4)),
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
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::TakeAnchor,
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
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(NodeType::Raw, NodeType::File, mark))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::TakeAnchor,
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
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::TakeAnchor,
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
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::TakeAnchor,
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
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(NodeType::Map, NodeType::File, mark))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        view.anchor_name(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::TakeAnchor,
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
    assert!(!view.is_file());
    assert!(view.is_take_anchor());
    assert!(!view.is_get_anchor());

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
        view.file(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::File,
            mark
        ))
    );
    if let Node::TakeAnchor(node) = &data.get(7).node {
        assert_eq!(
            view.take_anchor(),
            Ok(TakeAnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(view.anchor_name(), Ok("anchor"));
}

#[test]
fn test_file() {
    let data = test_data();
    let view = View::new(data.get(6), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::File);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(view.is_tagged());
    assert!(view.is_file());
    assert!(view.is_take_anchor());
    assert!(!view.is_get_anchor());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(NodeType::File, NodeType::Raw, mark))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::File,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::File,
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
    if let Node::File(node) = &data.get(6).node {
        assert_eq!(
            view.file(),
            Ok(FileView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    if let Node::TakeAnchor(node) = &data.get(7).node {
        assert_eq!(
            view.take_anchor(),
            Ok(TakeAnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::File,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(view.anchor_name(), Ok("anchor"));
}

#[test]
fn test_take_anchor() {
    let data = test_data();
    let view = View::new(data.get(7), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::TakeAnchor);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_file());
    assert!(view.is_take_anchor());
    assert!(!view.is_get_anchor());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
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
            NodeType::TakeAnchor,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.file(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::File,
            mark
        ))
    );
    if let Node::TakeAnchor(node) = &data.get(7).node {
        assert_eq!(
            view.take_anchor(),
            Ok(TakeAnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a take anchor");
    }
    assert_eq!(
        view.get_anchor(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(view.anchor_name(), Ok("anchor"));
}

#[test]
fn test_get_anchor() {
    let data = test_data();
    let view = View::new(data.get(8), &data, ());
    let mark = Mark::default();

    assert_eq!(view.mark(), mark);
    assert_eq!(view.node_type(), NodeType::GetAnchor);

    assert!(!view.is_null());
    assert!(!view.is_raw());
    assert!(!view.is_string());
    assert!(!view.is_list());
    assert!(view.is_map());
    assert!(!view.is_tagged());
    assert!(!view.is_file());
    assert!(!view.is_take_anchor());
    assert!(view.is_get_anchor());

    assert_eq!(
        view.raw(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(
        view.string(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        view.list(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
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
            NodeType::GetAnchor,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        view.file(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        view.take_anchor(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::TakeAnchor,
            mark
        ))
    );
    if let Node::GetAnchor(node) = &data.get(8).node {
        assert_eq!(
            view.get_anchor(),
            Ok(GetAnchorView::new(Default::default(), node, &data, ()))
        );
    } else {
        panic!("The node is not a get anchor");
    }
    assert_eq!(view.anchor_name(), Ok("anchor"));
}
