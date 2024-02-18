use regex::Regex;

#[derive(Debug, Clone)]
pub struct ElementDiv {
    x: usize,
    y: usize,
    html: String,
    width: String,
    height: String,
    background_color: String,
    pub children: Vec<ElementDiv>,
}

impl ElementDiv {
    pub fn is_match_open(html: &str) -> bool {
        let regex = Regex::new(r"^\s*<div\s*(style=[\s\S]*?)?\s*>$").unwrap();
        regex.is_match(html)
    }
    pub fn is_match_close(html: &str) -> bool {
        let regex = Regex::new(r"^\s*</div>$").unwrap();
        regex.is_match(html)
    }
    pub fn new() -> Self {
        ElementDiv {
            x: 0,
            y: 0,
            html: "".to_string(),
            children: vec![],
            width: "".to_string(),
            height: "".to_string(),
            background_color: "".to_string(),
        }
    }
    pub fn add_child(&mut self, child: ElementDiv) {
        self.children.push(child)
    }
    pub fn set_html(&mut self, html: String) {
        self.html = html
    }
    pub fn parse(&mut self) {
        let (width, height, background_color) = self.parse_attr(&self.html);
        self.width = width;
        self.height = height;
        self.background_color = background_color;
    }
    pub fn parse_attr(&self, attr_str: &String) -> (String, String, String) {
        let background_color_regex = Regex::new(r"\s*background-color:\s*(#[0-9a-f]{6});?")
            .expect("wrong background-color regexp");
        let width_regex = Regex::new(r"\s*width:\s*(\d+)(px);?").expect("wrong width regexp");
        let height_regex = Regex::new(r"\s*height:\s*(\d+)(px);?").expect("wrong height regexp");

        let mut bg_color = "#fff".to_string();
        if let Some(f1) = background_color_regex.captures(attr_str.as_str()) {
            bg_color = f1[1].to_string();
        };

        let mut w = "auto".to_string();
        if let Some(f2) = width_regex.captures(attr_str.as_str()) {
            w = f2[1].to_string();
        };

        let mut h = "auto".to_string();
        if let Some(f3) = height_regex.captures(attr_str.as_str()) {
            h = f3[1].to_string();
        };
        (w, h, bg_color)
    }
}
