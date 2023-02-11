use crate::node_kind::NodeKind;

pub struct Node {
    pub parent: Option<Box<Node>>,
    pub children: Vec<Box<Node>>,
    pub data: String,
    pub kind: NodeKind,
}

impl Node {
    pub fn new(data: String, kind: NodeKind) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            data: data,
            kind,
        }
    }

    pub fn add_child(&mut self, node: Box<Node>) {
        self.children.push(node);
    }

    pub fn print(&self) {
        print!("{}-", self.data);
        for child in &self.children {
            child.print();
        }
        println!("");
    }
}