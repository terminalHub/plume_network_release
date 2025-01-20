use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Payload {
    pub message: String,
    pub signature: String,
    pub address: String,
    pub twitter_encrypted_username: Option<String>,
    pub twitter_encrypted_id: Option<String>,
    pub discord_encrypted_username: Option<String>,
    pub discord_encrypted_id: Option<String>,
}