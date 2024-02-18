pub mod div;
pub mod parser;

// pub mod engine {
#[derive(Debug)]
pub struct Node {
    __id: u32,
    x: f32,
    y: f32,
    width: String,
    height: String,
    background_color: String,
    children: Vec<Node>,
}
impl Node {
    pub fn new(__id: u32, width: String, height: String, background_color: String) -> Self {
        Node {
            __id,
            x: 0f32,
            y: 0f32,
            width,
            height,
            background_color,
            children: vec![],
        }
    }
}

#[derive(Debug)]
pub struct LayoutEngine {
    root: Node,
}
impl LayoutEngine {
    pub fn new() -> Self {
        LayoutEngine {
            root: Node::new(0, "0".to_string(), "0".to_string(), String::from("#fff")),
        }
    }
    pub fn set_root(&mut self, root: Node) {
        self.root = root;
    }
}
// }
