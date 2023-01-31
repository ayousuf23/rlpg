pub struct Node {
    pub parent: Option<Box<Node>>,
    pub child: Option<Box<Node>>,
    pub data: String
}

impl Node {
    pub fn new(data: String) -> Self {
        Self {
            parent: None,
            child: None,
            data: data,
        }
    }
}