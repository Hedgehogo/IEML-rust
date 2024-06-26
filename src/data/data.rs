use super::node::node::MarkedNode;
use super::view::view::View;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub(crate) data: Vec<MarkedNode>,
}

impl Data {
    #[allow(dead_code)]
    pub(crate) fn new<const N: usize>(data: [MarkedNode; N]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }

    pub(crate) fn get(&self, index: usize) -> &MarkedNode {
        self.data
            .get(index)
            .expect("Incorrect document structure, node does not exist.")
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> &mut MarkedNode {
        self.data
            .get_mut(index)
            .expect("Incorrect document structure, node does not exist.")
    }

    pub fn view(&self) -> View {
        View::new(
            self.data
                .last()
                .expect("Incorrect document structure, node does not exist."),
            self,
        )
    }
}
