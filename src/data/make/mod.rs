pub mod maker;
pub mod error;
mod init;

use std::{
    collections::HashMap,
    path::PathBuf,
    error::Error,
};
use super::{
    mark::Mark,
    cell::{TagCell, FileCell, AnchorCell, DataCell, Data},
};
use maker::Maker;
use init::init;
use error::*;

pub type MakeResult<E> = Result<(), marked::MakeError<E>>;

pub fn null<E>(mark: Mark) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
{
    move |maker| {
        maker.add(mark, DataCell::Null);
        Ok(())
    }
}

pub fn raw<E, S>(mark: Mark, raw: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        maker.add(mark, DataCell::Raw(raw.into()));
        Ok(())
    }
}

pub fn string<E, S>(mark: Mark, string: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        Ok(maker.add(mark, DataCell::String(string.into())))
    }
}

pub fn list<E, F, I>(mark: Mark, iter: I) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    I: Iterator<Item = F>,
{
    move |maker| {
        let result: Result<_, _> = iter.map(|f| {
            f(maker).map(|_| maker.last())
        }).collect();
        result.map(|i| maker.add(mark, DataCell::List(i)))
    }
}

pub fn map<E, F, S, I>(mark: Mark, iter: I) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, F)>,
{
    move |maker| {
        let result: Result<_, _> = iter.map(|(key, f)| {
            f(maker).map(|_| (key.into(), maker.last()))
        }).collect();
        result.map(|i| maker.add(mark, DataCell::Map(i)))
    }
}

pub fn tag<E, F, S>(mark: Mark, tag: S, f: F) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
{
    move |maker| {
        f(maker)?;
        let result = TagCell { tag: tag.into(), cell_index: maker.last() };
        maker.add(mark, DataCell::Tag(result));
        Ok(())
    }
}

pub fn file<E, F, A, S, I>(mark: Mark, path: PathBuf, anchors: I, f: F) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    A: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    move |maker| {
        let file_anchors = anchors.map(|(key, f)| {
            f(maker).map(|_| (key.into(), maker.last()))
        }).collect::<Result<_, _>>()?;
        let result = maker.child(|maker| {
            f(maker).map(|_| FileCell {
                path,
                cell_index: maker.last(),
                anchors: std::mem::take(maker.anchors()),
                file_anchors,
                ..Default::default()
            })
        })?;
        maker.add(mark, DataCell::File(result));
        Ok(())
    }
}

pub fn take_anchor<E, F, S>(mark: Mark, name: S, f: F) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
{
    move |maker| {
        f(maker)?;
        let name = name.into();
        let result = AnchorCell { name: name.clone(), cell_index: maker.last() };
        maker.add_anchor(name.clone(), maker.last()).ok_or(
            marked::MakeError::new(mark, MakeError::new(maker.path().to_path_buf(), MakeErrorReason::AnchorAlreadyExist(name)))
        )?;
        maker.add(mark, DataCell::TakeAnchor(result));
        Ok(())
    }
}

pub fn get_anchor<E, S>(mark: Mark, name: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        let result = AnchorCell { name: name.into(), cell_index: 0 };
        maker.add(mark, DataCell::TakeAnchor(result));
        Ok(())
    }
}

pub fn make<E, F>(mark: Mark, f: F) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
{
    let mut data = Data::default();
    let mut maker = Maker::new(&mut data, PathBuf::new());
    let result = maker.child(|maker| {
        f(maker).map(|_| FileCell {
            cell_index: maker.last(),
            anchors: std::mem::take(maker.anchors()),
            ..Default::default()
        })
    })?;
    maker.add(mark, DataCell::File(result));
    data.index = maker.last();
    init(&mut data)?;
    Ok(data)
}

pub fn make_file<E, F, A, S, I>(mark: Mark, path: PathBuf, anchors: I, f: F) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    A: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    let mut data = Data::default();
    let mut maker = Maker::new(&mut data, path.clone());
    file(mark, path, anchors, f)(&mut maker)?;
    data.index = maker.last();
    init(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use super::super::{
        node::Node,
        node_type::NodeType,
    };
    use super::*;
    
    #[test]
    fn test_null() {
        let data = make::<Infallible, _>(Mark::default(), null(Mark::default())).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::Null);
    }
    
    #[test]
    fn test_raw() {
        let data = make::<Infallible, _>(Mark::default(), raw(Mark::default(), "hello")).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::Raw);
        assert_eq!(clear_node.raw(), Ok("hello"));
    }
    
    #[test]
    fn test_string() {
        let data = make::<Infallible, _>(Mark::default(), string(Mark::default(), "hello")).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::String);
        assert_eq!(clear_node.string(), Ok("hello"));
    }
    
    #[test]
    fn test_list() {
        let data = make::<Infallible, _>(Mark::default(), {
            list(Mark::default(), Vec::from([
                Box::new(raw(Mark::default(), "hello")) as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>,
                Box::new(string(Mark::default(), "hello")),
            ]).into_iter())
        }).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::List);
        
        let mut list_iter = clear_node.list_iter().unwrap();
        assert_eq!(list_iter.next().unwrap().node_type(), NodeType::Raw);
        assert_eq!(list_iter.next().unwrap().node_type(), NodeType::String);
        assert!(list_iter.next().is_none());
    }
    
    #[test]
    fn test_map() {
        let data = make::<Infallible, _>(Mark::default(), {
            map(Mark::default(), HashMap::from([
                ("first", Box::new(raw(Mark::default(), "hello")) as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>),
                ("second", Box::new(string(Mark::default(), "hello"))),
            ]).into_iter())
        }).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::Map);
        
        let mut map: Vec<(&String, Node)> = clear_node.map_iter().unwrap().collect();
        map.sort_by(|i, j| i.0.cmp(&j.0));
        assert_eq!(map.len(), 2);
        assert_eq!(map[0].0, "first");
        assert_eq!(map[0].1.node_type(), NodeType::Raw);
        assert_eq!(map[1].0, "second");
        assert_eq!(map[1].1.node_type(), NodeType::String);
    }
    
    #[test]
    fn test_tag() {
        let data = make::<Infallible, _>(Mark::default(), {
            tag(Mark::default(), "tag", null(Mark::default()))
        }).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::Tag);
        assert_eq!(node.tag(), Ok("tag"));
        
        assert!(node.is_null());
    }
    
    #[test]
    fn test_file() {
        let data = make::<Infallible, _>(Mark::default(), {
            file(Mark::default(), "dir/name.ieml".into(), HashMap::from([
                ("file-anchor", Box::new(null(Mark::default())) as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>)
            ]).into_iter(), raw(Mark::default(), "hello"))
        }).unwrap();
        let node = data.node();
        let clear_node = node.clear_step_file().unwrap();
        
        assert_eq!(clear_node.node_type(), NodeType::File);
        assert_eq!(clear_node.file_path(), Ok(PathBuf::from("dir/name.ieml").as_path()));
        
        let anchors: Vec<_> = clear_node.file_anchors().unwrap().file_anchors_iter().collect();
        assert_eq!(anchors.len(), 1);
        assert_eq!(anchors[0].0, "file-anchor");
        
        assert!(clear_node.is_raw());
        assert_eq!(clear_node.raw(), Ok("hello"));
    }
}