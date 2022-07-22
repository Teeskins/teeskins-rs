use std::vec;

use serde::{Serialize,Deserialize};

use crate::data::{Asset, Assets, AssetType};
use crate::utils::get_request;

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
    /// In case of error it return None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// 
    /// const ASSET_ID: i32 = 9;
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     "https://api.skins.tw".to_string()
    /// );
    /// 
    /// let asset = api.get_asset_by_id(ASSET_ID);
    /// 
    /// match asset {
    ///     Some(v) => assert_eq!(v.id, ASSET_ID), 
    ///     None => assert_eq!(-1, ASSET_ID),
    /// }
    /// ```

    pub fn get_asset_by_id(
        &self,
        id: i32
    ) -> Option<Asset> {
        let url = format!("{}/api/asset/{}", self.host, id);
        let res = get_request::<Asset>(&url);

        match res {
            Ok(asset) => Some(asset),
            Err(_) => None
        }
    }

    /// Get n assets of a specifiq type
    /// In case of error it return None
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use teeskins_rs::api::TeeskinsApi;
    /// use teeskins_rs::data::AssetType;
    /// 
    /// const LIMIT: i32 = 5;
    /// 
    /// let api = TeeskinsApi::new(
    ///     "".to_string(),
    ///     "https://api.skins.tw".to_string()
    /// );
    /// 
    /// let asset = api.get_assets_by_type(AssetType::SKIN, LIMIT);
    /// 
    /// match asset {
    ///     Some(v) => {
    ///         assert_eq!(v.len() as i32, LIMIT)
    /// },
    ///     None => assert_eq!(0, LIMIT),
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

        let res = get_request::<Assets>(&url);

        match res {
            Ok(valid_assets) => Some(valid_assets),
            Err(_) => None
        }
    }
}
