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

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_descr(&self) -> &str {
        self.descr.as_str()
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_descr(&mut self, value: &str) {
        self.descr = value.to_owned();
    }
}
