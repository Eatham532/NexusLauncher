use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: i64,
    pub secured: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "legacyUser")]
    pub legacy_user: bool,
    #[serde(rename = "verifiedByParent")]
    pub verified_by_parent: bool,
    pub hashed: bool,
}