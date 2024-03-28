use super::super::cell::data_cell::{
    DataCell, FileCell, GetAnchorCell, ListCell, MapCell, MarkedDataCell, TaggedCell,
    TakeAnchorCell,
};
use super::*;
use std::{collections::HashMap, path::PathBuf};

fn test_data() -> Data {
    Data::new(
        6,
        [
            (
                0,
                MarkedDataCell::new(DataCell::Null, Mark { line: 2, symbol: 5 }),
            ),
            (
                1,
                MarkedDataCell::new(DataCell::Raw("hello".into()), Default::default()),
            ),
            (
                2,
                MarkedDataCell::new(DataCell::String("hello".into()), Default::default()),
            ),
            (
                3,
                MarkedDataCell::new(
                    DataCell::List(ListCell::new(vec![0, 1])),
                    Default::default(),
                ),
            ),
            (
                4,
                MarkedDataCell::new(
                    DataCell::Map(MapCell::new(HashMap::from([
                        ("first".to_string(), 2),
                        ("second".to_string(), 3),
                        ("third".to_string(), 8),
                    ]))),
                    Default::default(),
                ),
            ),
            (
                5,
                MarkedDataCell::new(
                    DataCell::Tagged(TaggedCell::new("tag".into(), 7)),
                    Default::default(),
                ),
            ),
            (
                6,
                MarkedDataCell::new(
                    DataCell::File(FileCell {
                        cell_index: 5,
                        path: PathBuf::from("dir/name.ieml"),
                        anchors: Default::default(),
                        file_anchors: Default::default(),
                        parent: None,
                    }),
                    Default::default(),
                ),
            ),
            (
                7,
                MarkedDataCell::new(
                    DataCell::TakeAnchor(TakeAnchorCell::new("anchor".into(), 4)),
                    Default::default(),
                ),
            ),
            (
                8,
                MarkedDataCell::new(
                    DataCell::GetAnchor(GetAnchorCell::new("anchor".into(), 4)),
                    Default::default(),
                ),
            ),
        ],
    )
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
    let node = Node::new(data.get(0), &data);
    let mark = Mark { line: 2, symbol: 5 };

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::Null);

    assert!(node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(!node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::Null, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::List,
            mark
        ))
    );
    assert_eq!(
        node.map(),
        Err(make_another_type_error(NodeType::Null, NodeType::Map, mark))
    );
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::Null,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        node.anchor_name(),
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
    let node = Node::new(data.get(1), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::Raw);

    assert!(!node.is_null());
    assert!(node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(!node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(node.raw(), Ok("hello"));
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::Raw, NodeType::List, mark))
    );
    assert_eq!(
        node.map(),
        Err(make_another_type_error(NodeType::Raw, NodeType::Map, mark))
    );
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(NodeType::Raw, NodeType::File, mark))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::Raw,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        node.anchor_name(),
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
    let node = Node::new(data.get(2), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::String);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(node.is_string());
    assert!(!node.is_list());
    assert!(!node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Raw,
            mark
        ))
    );
    assert_eq!(node.string(), Ok("hello"));
    assert_eq!(
        node.list(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::List,
            mark
        ))
    );
    assert_eq!(
        node.map(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Map,
            mark
        ))
    );
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::String,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        node.anchor_name(),
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
    let node = Node::new(data.get(3), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::List);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(node.is_list());
    assert!(!node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::List, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::String,
            mark
        ))
    );
    if let DataCell::List(cell) = &data.get(3).cell {
        assert_eq!(
            node.list(),
            Ok(ListNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a list");
    }
    assert_eq!(
        node.map(),
        Err(make_another_type_error(NodeType::List, NodeType::Map, mark))
    );
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::File,
            mark
        ))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::List,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        node.anchor_name(),
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
    let node = Node::new(data.get(4), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::Map);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::Map, NodeType::List, mark))
    );
    if let DataCell::Map(cell) = &data.get(4).cell {
        assert_eq!(
            node.map(),
            Ok(MapNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a map");
    }
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(NodeType::Map, NodeType::File, mark))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::TakeAnchor,
            mark
        ))
    );
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::Map,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(
        node.anchor_name(),
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
    let node = Node::new(data.get(5), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::Tagged);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(node.is_tagged());
    assert!(!node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::Tagged, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::Tagged, NodeType::List, mark))
    );
    if let DataCell::Map(cell) = &data.get(4).cell {
        assert_eq!(
            node.map(),
            Ok(MapNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a map");
    }
    if let DataCell::Tagged(cell) = &data.get(5).cell {
        assert_eq!(
            node.tagged(),
            Ok(TaggedNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a tagged");
    }
    assert_eq!(
        node.file(),
        Err(make_another_type_error(NodeType::Tagged, NodeType::File, mark))
    );
    if let DataCell::TakeAnchor(cell) = &data.get(7).cell {
        assert_eq!(
            node.take_anchor(),
            Ok(TakeAnchorNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a take anchor");
    }
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::Tagged,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(node.anchor_name(), Ok("anchor"));
}

#[test]
fn test_file() {
    let data = test_data();
    let node = Node::new(data.get(6), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::File);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(node.is_tagged());
    assert!(node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::File, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::File,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::File, NodeType::List, mark))
    );
    if let DataCell::Map(cell) = &data.get(4).cell {
        assert_eq!(
            node.map(),
            Ok(MapNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a map");
    }
    if let DataCell::Tagged(cell) = &data.get(5).cell {
        assert_eq!(
            node.tagged(),
            Ok(TaggedNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a tagged");
    }
    if let DataCell::File(cell) = &data.get(6).cell {
        assert_eq!(
            node.file(),
            Ok(FileNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a take anchor");
    }
    if let DataCell::TakeAnchor(cell) = &data.get(7).cell {
        assert_eq!(
            node.take_anchor(),
            Ok(TakeAnchorNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a take anchor");
    }
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::File,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(node.anchor_name(), Ok("anchor"));
}

#[test]
fn test_take_anchor() {
    let data = test_data();
    let node = Node::new(data.get(7), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::TakeAnchor);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::TakeAnchor, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::TakeAnchor, NodeType::List, mark))
    );
    if let DataCell::Map(cell) = &data.get(4).cell {
        assert_eq!(
            node.map(),
            Ok(MapNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a map");
    }
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(NodeType::TakeAnchor, NodeType::File, mark))
    );
    if let DataCell::TakeAnchor(cell) = &data.get(7).cell {
        assert_eq!(
            node.take_anchor(),
            Ok(TakeAnchorNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a take anchor");
    }
    assert_eq!(
        node.get_anchor(),
        Err(make_another_type_error(
            NodeType::TakeAnchor,
            NodeType::GetAnchor,
            mark
        ))
    );
    assert_eq!(node.anchor_name(), Ok("anchor"));
}

#[test]
fn test_get_anchor() {
    let data = test_data();
    let node = Node::new(data.get(8), &data);
    let mark = Mark::default();

    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::GetAnchor);

    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(!node.is_tagged());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(node.is_get_anchor());

    assert_eq!(
        node.raw(),
        Err(make_another_type_error(NodeType::GetAnchor, NodeType::Raw, mark))
    );
    assert_eq!(
        node.string(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::String,
            mark
        ))
    );
    assert_eq!(
        node.list(),
        Err(make_another_type_error(NodeType::GetAnchor, NodeType::List, mark))
    );
    if let DataCell::Map(cell) = &data.get(4).cell {
        assert_eq!(
            node.map(),
            Ok(MapNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a map");
    }
    assert_eq!(
        node.tagged(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::Tagged,
            mark
        ))
    );
    assert_eq!(
        node.file(),
        Err(make_another_type_error(NodeType::GetAnchor, NodeType::File, mark))
    );
    assert_eq!(
        node.take_anchor(),
        Err(make_another_type_error(
            NodeType::GetAnchor,
            NodeType::TakeAnchor,
            mark
        ))
    );
    if let DataCell::GetAnchor(cell) = &data.get(8).cell {
        assert_eq!(
            node.get_anchor(),
            Ok(GetAnchorNode::new(Default::default(), cell, &data))
        );
    } else {
        panic!("The cell is not a get anchor");
    }
    assert_eq!(node.anchor_name(), Ok("anchor"));
}
