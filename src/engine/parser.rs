use std::{
    borrow::{Borrow, BorrowMut},
    ops::Add,
    rc::Rc,
};

use regex::Regex;
use scraper::node::Element;

use crate::engine::Node;

use super::div::ElementDiv;

pub struct Parser {
    __id: u32,
}

impl Parser {
    pub fn new() -> Self {
        Parser { __id: 0 }
    }
    pub fn parse(&mut self, html: String) {
        let reg = Regex::new(r"<body\s*([\s\S]*?)\s*>([\s\S]*)<\/body>").unwrap();
        let caps = reg.captures(html.as_str()).unwrap();
        let attr = &caps[1];
        let child_html = &caps[2];

        let (width, height, background_color) = self.parse_attr(&attr.to_string());

        self.__id += 1;
        // let mut root = Node::new(self.__id, width, height, background_color);
        let root = ElementDiv::new();
        let mut stack: Vec<ElementDiv> = Vec::new();
        stack.push(root);
        self.analyze_tag(child_html, &mut stack);
        let mut body = stack.pop().unwrap();
        self.parse_tree(&mut body);
        println!("{:#?}", &body);
    }
    pub fn layout() {}
    fn parse_tree(&self, root: &mut ElementDiv) {
        root.parse();
        root.children
            .iter_mut()
            .for_each(|mut n| self.parse_tree(&mut n))
    }

    fn analyze_tag<'a>(&self, html: &str, stack: &mut Vec<ElementDiv>) {
        // let mut cur_element: Option<&ElementDiv> = None;
        let mut i = 0;
        let mut input = "".to_string();
        while let Some(c) = html.chars().nth(i) {
            input = input.add(&c.to_string());
            if ElementDiv::is_match_open(input.as_str()) {
                let mut elem = ElementDiv::new();
                elem.set_html(input.clone());
                input.clear();
                stack.push(elem);
            }
            if c.eq(&'>') {
                if ElementDiv::is_match_close(input.as_str()) {
                    let child = stack.pop().unwrap();
                    let parent = stack.last_mut().unwrap();
                    parent.add_child(child);
                    // println!("{:#?}", &parent);
                    input.clear();
                }
            }
            i += 1;
        }
    }
    fn parse_attr(&self, attr_str: &String) -> (String, String, String) {
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
