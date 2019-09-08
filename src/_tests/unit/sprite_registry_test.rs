use std::collections::HashMap;

use raylib::Rectangle;
use xmlparser as xml;

use crate::raylib::sprite_registry::SpriteRegistry;

#[ignore]
#[test]
fn get_source_rec_by_name() {
    let xml = r#"<TextureAtlas imagePath="sheet.png">
                    <SubTexture name="beam0.png" x="143" y="377" width="43" height="31"/>
                    <SubTexture name="beam1.png" x="327" y="644" width="40" height="20"/>
                    <SubTexture name="beam2.png" x="262" y="907" width="38" height="31"/>
                    <SubTexture name="beam3.png" x="396" y="384" width="29" height="29"/>
                    <SubTexture name="beam4.png" x="177" y="496" width="41" height="17"/>
                </TextureAtlas>
                "#;

    let registry = SpriteRegistry::from_xml(xml.to_string());

    let rec = registry.get_source_rec("beam02.png");

    assert_eq!(rec.x, 262.0);
    assert_eq!(rec.y, 907.0);
    assert_eq!(rec.width, 38.0);
    assert_eq!(rec.height, 31.0)
}

#[test]
fn xml_parsing() {
    let xml = r#"<TextureAtlas imagePath="sheet.png">
                    <SubTexture name="beam0.png" x="143" y="377" width="43" height="31"/>
                    <SubTexture name="beam1.png" x="327" y="644" width="40" height="20"/>
                </TextureAtlas>
                "#;


    let mut current_rec_opt: Option<(String, Rectangle)> = Option::None;


    let mut recs: HashMap<String, Rectangle> = HashMap::with_capacity(100);

    let mut current_name = String::from("");
    let mut current_rec = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };


    for token in xmlparser::Tokenizer::from(xml) {
        match token {
            Ok(xml::Token::Attribute { prefix, local, value, span }) => {
                match local.as_str() {
                    "name" => {
                        if current_name.to_string() != String::from("") {
                            recs.insert(current_name.to_string(), current_rec);
                        }
                        current_name = value.as_str().to_string();
                        current_rec = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
                    }
                    "x" => { current_rec.x = value.as_str().parse().unwrap() }
                    "y" => { current_rec.y = value.as_str().parse().unwrap() }
                    "width" => { current_rec.width = value.as_str().parse().unwrap() }
                    "height" => { current_rec.height = value.as_str().parse().unwrap() }

                    _ => {}
                }
            }
            _ => {}
        }
    }

    recs.insert(current_name.to_string(), current_rec);

    assert_eq!(recs.len(), 2);

    let beam0 = recs.get("beam0.png").unwrap();
    assert_eq!(beam0.x, 143.0);
    assert_eq!(beam0.y, 377.0);
    assert_eq!(beam0.width, 43.0);
    assert_eq!(beam0.height, 31.0);

    let beam1 = recs.get("beam1.png").unwrap();
    assert_eq!(beam1.x, 327.0);
    assert_eq!(beam1.y, 644.0);
    assert_eq!(beam1.width, 40.0);
    assert_eq!(beam1.height, 20.0);


//    assert_eq!(1, 2)
}