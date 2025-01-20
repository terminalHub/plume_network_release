use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    pub registered: bool,
}