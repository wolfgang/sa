struct Test1 {
    x: u32
}

impl Test1 {
    pub fn inc(&mut self) {
        self.x += 1;
        self.inc_2();
    }

    fn inc_2(&mut self) {
        self.x += 2
    }
}

#[test]
fn nothing() {
    let mut obj = Test1 { x: 11 };
    obj.inc();
    assert_eq!(obj.x, 14);
}