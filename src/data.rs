use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Asset {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub thumbnail_path: String,
    pub mime_type: String,
    pub author: String,
    #[serde(alias = "type")]
    pub asset_type: String,
    pub md5_checksum: String,
    #[serde(alias = "isPublic")]
    pub is_public: i32,
    pub created_at: String,
    pub updated_at: String
}

pub type Assets = Vec<Asset>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Uploader {
    pub id: i32,
    pub name: String,
    pub profile_photo_path: Option<String>,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub profile_photo_url: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
    pub profile_photo_url: String,
    pub member_since: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssetsCount {
    pub skins: i32,
    pub mapres: i32,
    pub gameskins: i32,
    pub emoticons: i32,
    pub cursors: i32,
    pub particles: i32,
    pub entities: i32,
    pub fonts: i32,
    pub grid_template: i32
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserData {
    pub rank: i32,
    pub total_count: i32,
    pub data: AssetsCount
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Profile {
    pub user: User,
    pub upload_data: UserData,
    pub download_data: UserData
}
