use crate::sprite_registry::SpriteRegistry;

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

    let beam2 = registry.get_source_rec("beam2.png");
    assert_eq!(beam2.x, 262.0);
    assert_eq!(beam2.y, 907.0);
    assert_eq!(beam2.width, 38.0);
    assert_eq!(beam2.height, 31.0);

    let beam4 = registry.get_source_rec("beam4.png");
    assert_eq!(beam4.x, 177.0);
    assert_eq!(beam4.y, 496.0);
    assert_eq!(beam4.width, 41.0);
    assert_eq!(beam4.height, 17.0);
}
