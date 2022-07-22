use serde::{Serialize, Deserialize};

use crate::data::{Asset};

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

    pub fn get_asset_by_id(&self, id: i32) -> Result<Asset, ureq::Error> {
        let url = format!(
            "{host}/api/asset/{id}",
            host=self.host,
            id=id
        );
        let res: serde_json::Value = ureq::get(&url)
            .call()?
            .into_json()?;

        let ret = serde_json::from_value(res);

        match ret {
            Ok(asset) => Ok(asset),
            Err(_) => Ok(Asset::default())
        }
    }
}
