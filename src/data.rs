#[derive(Hash, PartialEq, Eq)]
pub struct Key {
    pub id: String,
    pub obj_type: String,
    pub ldn: String,
}

impl Key {
    pub fn make_filepath(&self) -> String {
        format!("output/{}-{}-{}.csv", self.obj_type, self.ldn, self.id)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Data {
    pub time: String,
    pub value: String,
}

