#[derive(Debug)]
pub struct Item {
    id: i64,
    descr: String,
    amount: f64,
    active: bool,
    picture: Vec<u8>
}

impl Item {
    pub fn new(id: i64, descr: &str, amount: f64, active: bool, picture: Option<Vec<u8>>) -> Self {
        Item {
            id,
            descr: descr.to_owned(),
            amount,
            active,
            picture: picture.unwrap_or_default()
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_descr(&self) -> &str {
        self.descr.as_str()
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn get_picture(&self) -> &Vec<u8> {
        self.picture.as_ref()
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_descr(&mut self, value: &str) {
        self.descr = value.to_owned();
    }

    pub fn set_amount(&mut self, value: f64) {
        self.amount = value;
    }

    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    pub fn set_picture(&mut self, value: &Vec<u8>) {
        self.picture = value.to_vec();
    }
}
