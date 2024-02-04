pub struct Item {
    id: i64,
    descr: String
}

impl Item {
    pub fn new(id: i64, descr: &str) -> Self {
        Item {
            id,
            descr: descr.to_owned()
        }
    }

    pub fn get_descr(&self) -> &str {
        self.descr.as_str()
    }
}
