use std::collections::HashMap;

use raylib::Rectangle;
use xmlparser as xml;
use xmlparser::StrSpan;

pub struct SpriteRegistry {
    recs: HashMap<String, Rectangle>
}

impl SpriteRegistry {
    pub fn from_xml(source_xml: String) -> Self {
        Self { recs: Self::parse(source_xml) }
    }

    fn parse(source_xml: String) -> HashMap<String, Rectangle> {
        let mut recs: HashMap<String, Rectangle> = HashMap::with_capacity(100);
        let mut current_name = String::from("");
        let mut current_rec = Self::new_rec();


        for token in xmlparser::Tokenizer::from(source_xml.as_str()) {
            match token {
                Ok(xml::Token::Attribute { prefix: _, local, value, span: _ }) => {
                    match local.as_str() {
                        "name" => {
                            if !current_name.is_empty() {
                                recs.insert(current_name.to_string(), current_rec);
                            }
                            current_name = value.to_string();
                            current_rec = Self::new_rec();
                        }
                        "x" => { current_rec.x = Self::value_to_float(value) }
                        "y" => { current_rec.y = Self::value_to_float(value) }
                        "width" => { current_rec.width = Self::value_to_float(value) }
                        "height" => { current_rec.height = Self::value_to_float(value) }

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        recs.insert(current_name.to_string(), current_rec);
        recs

    }

    pub fn get_source_rec(&self, name: &str) -> Rectangle {
        self.recs.get(name).unwrap().clone()
    }

    fn value_to_float(value: StrSpan) -> f32 {
        value.as_str().parse().unwrap()
    }

    fn new_rec() -> Rectangle {
        Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}