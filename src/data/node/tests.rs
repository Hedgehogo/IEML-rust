use std::path::PathBuf;
use super::super::cell::{DataCell, MarkedDataCell, MapCell, TagCell, FileCell, AnchorCell};
use super::*;

fn test_data() -> Data {
    Data::new(6, [
        (0, MarkedDataCell { cell: DataCell::Null, mark: Mark { line: 2, symbol: 5 } }),
        (1, MarkedDataCell { cell: DataCell::Raw("hello".into()), mark: Default::default() }),
        (2, MarkedDataCell { cell: DataCell::String("hello".into()), mark: Default::default() }),
        (3, MarkedDataCell { cell: DataCell::List(vec![0, 1]), mark: Default::default() }),
        (4, MarkedDataCell { cell: DataCell::Map(MapCell::from([
            ("first".to_string(), 2),
            ("second".to_string(), 3),
            ("third".to_string(), 8),
        ])), mark: Default::default() }),
        (5, MarkedDataCell { cell: DataCell::Tag(TagCell {
            cell_index: 7,
            tag: "tag".into() }),
            mark: Default::default()
        }),
        (6, MarkedDataCell { cell: DataCell::File(FileCell {
            cell_index: 5,
            path: PathBuf::from("dir/name.ieml"),
            anchors: Default::default(),
            file_anchors: Default::default(),
            parent: None
        }), mark: Default::default() }),
        (7, MarkedDataCell { cell: DataCell::TakeAnchor(AnchorCell {
            cell_index: 4,
            name: "anchor".into()
        }), mark: Default::default() }),
        (8, MarkedDataCell { cell: DataCell::GetAnchor(AnchorCell {
            cell_index: 4,
            name: "anchor".into()
        }), mark: Default::default() }),
    ])
}

fn make_another_type_error(node_type: NodeType, requested_type: NodeType, mark: Mark) -> marked::AnotherTypeError {
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Null, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Null, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Null, NodeType::List, mark)));
    assert_eq!(node.map_size(), Err(make_another_type_error(NodeType::Null, NodeType::Map, mark)));
    assert_eq!(node.size(), Err(make_another_type_error(NodeType::Null, NodeType::List, mark)));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::Null, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Null, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::Null, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Null, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Err(make_another_type_error(NodeType::Null, NodeType::TakeAnchor, mark)));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Null, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Null, NodeType::List, mark))));
    
    assert_eq!(node.map_iter().unwrap_err(), make_another_type_error(NodeType::Null, NodeType::Map, mark));
    assert_eq!(node.at("key"), Err(marked::MapError::NodeAnotherType(make_another_type_error(NodeType::Null, NodeType::Map, mark))));
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Ok("hello"));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Raw, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Raw, NodeType::List, mark)));
    assert_eq!(node.map_size(), Err(make_another_type_error(NodeType::Raw, NodeType::Map, mark)));
    assert_eq!(node.size(), Err(make_another_type_error(NodeType::Raw, NodeType::List, mark)));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::Raw, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Raw, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::Raw, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Raw, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Err(make_another_type_error(NodeType::Raw, NodeType::TakeAnchor, mark)));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Raw, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Raw, NodeType::List, mark))));
    
    assert_eq!(node.map_iter().unwrap_err(), make_another_type_error(NodeType::Raw, NodeType::Map, mark));
    assert_eq!(node.at("key"), Err(marked::MapError::NodeAnotherType(make_another_type_error(NodeType::Raw, NodeType::Map, mark))));
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::String, NodeType::Raw, mark)));
    assert_eq!(node.string(), Ok("hello"));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::String, NodeType::List, mark)));
    assert_eq!(node.map_size(), Err(make_another_type_error(NodeType::String, NodeType::Map, mark)));
    assert_eq!(node.size(), Err(make_another_type_error(NodeType::String, NodeType::List, mark)));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::String, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::String, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::String, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::String, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Err(make_another_type_error(NodeType::String, NodeType::TakeAnchor, mark)));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::String, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::String, NodeType::List, mark))));
    
    assert_eq!(node.map_iter().unwrap_err(), make_another_type_error(NodeType::String, NodeType::Map, mark));
    assert_eq!(node.at("key"), Err(marked::MapError::NodeAnotherType(make_another_type_error(NodeType::String, NodeType::Map, mark))));
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::List, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::List, NodeType::String, mark)));
    assert_eq!(node.list_size(), Ok(2));
    assert_eq!(node.map_size(), Err(make_another_type_error(NodeType::List, NodeType::Map, mark)));
    assert_eq!(node.size(), Ok(2));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::List, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::List, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::List, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::List, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Err(make_another_type_error(NodeType::List, NodeType::TakeAnchor, mark)));
    
    let mut list_iter = node.list_iter().unwrap();
    assert_eq!(list_iter.next().unwrap().node_type(), NodeType::Null);
    assert_eq!(list_iter.next().unwrap().node_type(), NodeType::Raw);
    assert_eq!(list_iter.next(), None);
    
    assert_eq!(node.at(0).unwrap().node_type(), NodeType::Null);
    assert_eq!(node.at(1).unwrap().node_type(), NodeType::Raw);
    assert_eq!(node.at(2), Err(marked::ListError::InvalidIndex(marked::InvalidIndexError::new(mark, InvalidIndexError::new(2, 2)))));
    
    assert_eq!(node.map_iter().unwrap_err(), make_another_type_error(NodeType::List, NodeType::Map, mark));
    assert_eq!(node.at("key"), Err(marked::MapError::NodeAnotherType(make_another_type_error(NodeType::List, NodeType::Map, mark))));
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Map, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Map, NodeType::List, mark)));
    assert_eq!(node.map_size(), Ok(3));
    assert_eq!(node.size(), Ok(3));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::Map, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Map, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::TakeAnchor, mark)));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Map, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Map, NodeType::List, mark))));
    
    let mut map = node.map_iter().unwrap().collect::<Vec<(&String, Node)>>();
    map.sort_by(|i, j| i.0.cmp(&j.0));
    assert_eq!(*map[0].0, "first".to_string());
    assert_eq!(map[0].1.node_type(), NodeType::String);
    assert_eq!(*map[1].0, "second".to_string());
    assert_eq!(map[1].1.node_type(), NodeType::List);
    assert_eq!(*map[2].0, "third".to_string());
    assert_eq!(map[2].1.node_type(), NodeType::GetAnchor);
    assert_eq!(map.get(3), None);
    
    assert_eq!(node.at("first").unwrap().node_type(), NodeType::String);
    assert_eq!(node.at("second").unwrap().node_type(), NodeType::List);
    assert_eq!(node.at("third").unwrap().node_type(), NodeType::GetAnchor);
    assert_eq!(node.at("key"), Err(marked::MapError::InvalidKey(marked::InvalidKeyError::new(mark, InvalidKeyError::new("key".to_string())))));
}

#[test]
fn test_tag() {
    let data = test_data();
    let node = Node::new(data.get(5), &data);
    let mark = Mark::default();
    
    assert_eq!(node.mark(), mark);
    assert_eq!(node.node_type(), NodeType::Tag);
    
    assert!(!node.is_null());
    assert!(!node.is_raw());
    assert!(!node.is_string());
    assert!(!node.is_list());
    assert!(node.is_map());
    assert!(node.is_tag());
    assert!(!node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Map, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Map, NodeType::List, mark)));
    assert_eq!(node.map_size(), Ok(3));
    assert_eq!(node.size(), Ok(3));
    assert_eq!(node.tag(), Ok("tag"));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Map, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Ok("anchor"));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Ok("anchor"));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Map, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Map, NodeType::List, mark))));
    
    assert!(node.map_iter().is_ok());
    assert!(node.at("first").is_ok());
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
    assert!(node.is_tag());
    assert!(node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Map, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Map, NodeType::List, mark)));
    assert_eq!(node.map_size(), Ok(3));
    assert_eq!(node.size(), Ok(3));
    assert_eq!(node.tag(), Ok("tag"));
    assert_eq!(node.file_path(), Ok(PathBuf::from("dir/name.ieml").as_path()));
    assert_eq!(node.take_anchor_name(), Ok("anchor"));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Ok("anchor"));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Map, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Map, NodeType::List, mark))));
    
    assert!(node.map_iter().is_ok());
    assert!(node.at("first").is_ok());
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(node.is_take_anchor());
    assert!(!node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Map, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Map, NodeType::List, mark)));
    assert_eq!(node.map_size(), Ok(3));
    assert_eq!(node.size(), Ok(3));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::Map, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Map, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Ok("anchor"));
    assert_eq!(node.get_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::GetAnchor, mark)));
    assert_eq!(node.anchor_name(), Ok("anchor"));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Map, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Map, NodeType::List, mark))));
    
    assert!(node.map_iter().is_ok());
    assert!(node.at("first").is_ok());
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
    assert!(!node.is_tag());
    assert!(!node.is_file());
    assert!(!node.is_take_anchor());
    assert!(node.is_get_anchor());
    
    assert_eq!(node.raw(), Err(make_another_type_error(NodeType::Map, NodeType::Raw, mark)));
    assert_eq!(node.string(), Err(make_another_type_error(NodeType::Map, NodeType::String, mark)));
    assert_eq!(node.list_size(), Err(make_another_type_error(NodeType::Map, NodeType::List, mark)));
    assert_eq!(node.map_size(), Ok(3));
    assert_eq!(node.size(), Ok(3));
    assert_eq!(node.tag(), Err(make_another_type_error(NodeType::Map, NodeType::Tag, mark)));
    assert_eq!(node.file_path(), Err(make_another_type_error(NodeType::Map, NodeType::File, mark)));
    assert_eq!(node.take_anchor_name(), Err(make_another_type_error(NodeType::Map, NodeType::TakeAnchor, mark)));
    assert_eq!(node.get_anchor_name(), Ok("anchor"));
    assert_eq!(node.anchor_name(), Ok("anchor"));
    
    assert_eq!(node.list_iter().unwrap_err(), make_another_type_error(NodeType::Map, NodeType::List, mark));
    assert_eq!(node.at(0), Err(marked::ListError::NodeAnotherType(make_another_type_error(NodeType::Map, NodeType::List, mark))));
    
    assert!(node.map_iter().is_ok());
    assert!(node.at("first").is_ok());
}