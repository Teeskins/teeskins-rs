use serde::{Serialize,Deserialize};

use crate::data::{
    Asset,
    Assets,
    AssetType,
    ForDiscord,
    Profile
};
use crate::api_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeskinsApi {
    token: String,
    host: String
}

impl Default for TeeskinsApi {
    fn default() -> TeeskinsApi {
        TeeskinsApi {
            token: "".to_string(),
            host: "http://localhost".to_string()
        }
    }
}

impl TeeskinsApi {
    pub fn new(token: String, host: String) -> TeeskinsApi {
        TeeskinsApi {
            token: token,
            host: host
        }
    }

    /// Get an asset with a specific id.
    /// In case of error it returns None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// use std::env;
    /// use dotenv::dotenv;
    /// 
    /// const ASSET_ID: i32 = 9;
    /// 
    /// dotenv().ok();
    /// 
    /// let host = env::var("HOST").unwrap();
    /// let discord_token = env::var("DISCORD_TOKEN").unwrap();
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     host
    /// );
    /// 
    /// let asset = api.get_asset_by_id(ASSET_ID);
    /// 
    /// match asset {
    ///     Some(v) => assert_eq!(v.id, ASSET_ID), 
    ///     None => assert_eq!(-1, ASSET_ID)
    /// }
    /// ```

    pub fn get_asset_by_id(
        &self,
        id: i32
    ) -> Option<Asset> {
        let url = format!(
            "{}/api/asset/{}",
            self.host,
            id
        );

        api_request::get::<Asset>(&url)
    }

    /// Get n assets of a specifiq type
    /// In case of error it returns None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// use teeskins_rs::data::AssetType;
    /// use std::env;
    /// use dotenv::dotenv;
    /// 
    /// const LIMIT: i32 = 5;
    /// 
    /// dotenv().ok();
    /// 
    /// let host = env::var("HOST").unwrap();
    /// let discord_token = env::var("DISCORD_TOKEN").unwrap();
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     host
    /// );
    /// 
    /// let asset = api.get_assets_by_type(AssetType::SKIN, LIMIT);
    /// 
    /// match asset {
    ///     Some(v) => assert_eq!(v.len() as i32, LIMIT),
    ///     None => assert_eq!(0, LIMIT)
    /// }
    /// ```
    
    pub fn get_assets_by_type(
        &self,
        asset_type: AssetType,
        limit: i32
    ) -> Option<Assets> {
        let url = format!(
            "{}/api/assets/{}/{}",
            self.host,
            asset_type,
            limit
        );

        api_request::get::<Assets>(&url)
    }

    /// This route has been made for discord user, to api_request::get
    /// the upload asset amount with a token
    /// In case of error it returns None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// use std::env;
    /// use dotenv::dotenv;
    /// 
    /// dotenv().ok();
    /// 
    /// let host = env::var("HOST").unwrap();
    /// let discord_token = env::var("DISCORD_TOKEN").unwrap();
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     host
    /// );
    /// 
    /// let asset = api.get_uploads_count_from_token(discord_token);
    /// ```
    
    pub fn get_uploads_count_from_token(
        &self,
        token: String
    ) -> Option<ForDiscord> {
        let url = format!(
            "{}/api/discord/{}",
            self.host,
            token
        );

       api_request::get::<ForDiscord>(&url)
    }

    /// Returns an user profile with a name
    /// In case of error it returns None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// use std::env;
    /// use dotenv::dotenv;
    /// 
    /// dotenv().ok();
    /// 
    /// let host = env::var("HOST").unwrap();
    /// let name = "nagi01".to_string();
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     host
    /// );
    /// 
    /// let asset = api.get_profile(name.clone());
    /// 
    /// match asset {
    ///     Some(v) => assert_eq!(v.user.name, name),
    ///     None => assert_eq!("wrong", name)
    /// }
    /// ```

    pub fn get_profile(
        &self,
        name: String
    ) -> Option<Profile> {
        let url = format!(
            "{}/api/profile/{}",
            self.host,
            name
        );

       api_request::get::<Profile>(&url)
    }
}
