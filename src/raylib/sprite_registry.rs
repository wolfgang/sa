use std::collections::HashMap;

use raylib::Rectangle;
use xmlparser as xml;
use xmlparser::StrSpan;

pub struct SpriteRegistry {
    recs: HashMap<String, Rectangle>
}

impl SpriteRegistry {
    pub fn from_xml(source_xml: String) -> Self {
        let mut registry = Self { recs: HashMap::with_capacity(100) };
        registry.parse(source_xml);
        registry
    }

    fn parse(&mut self, source_xml: String) {
        let mut current_name = String::from("");
        for token in xmlparser::Tokenizer::from(source_xml.as_str()) {
            match token {
                Ok(xml::Token::Attribute { prefix: _, local, value, span: _ }) => {
                    match local.as_str() {
                        "name" => {
                            self.recs.insert(value.to_string(), Self::new_rec());
                            current_name = value.to_string();
                        }
                        "x" => { self.current_rec(&current_name).x = Self::value_to_float(value) }
                        "y" => { self.current_rec(&current_name).y = Self::value_to_float(value) }
                        "width" => { self.current_rec(&current_name).width = Self::value_to_float(value) }
                        "height" => { self.current_rec(&current_name).height = Self::value_to_float(value) }

                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_source_rec(&self, name: &str) -> Rectangle {
        self.recs.get(name).unwrap().clone()
    }

    fn current_rec(&mut self, name: &String) -> &mut Rectangle {
        self.recs.get_mut(name.as_str()).unwrap()
    }

    fn value_to_float(value: StrSpan) -> f32 {
        value.as_str().parse().unwrap()
    }

    fn new_rec() -> Rectangle {
        Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}