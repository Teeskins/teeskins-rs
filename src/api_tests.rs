#[cfg(test)]
mod api_tests {
    use crate::api::TeeskinsApi;

    const ASSET_ID: i32 = 39;
    #[test]
    fn asset_by_id_test() {
        let api = TeeskinsApi::new(
            "".to_string(),
            "https://api.skins.tw".to_string()
        );

        let asset = api.get_asset_by_id(ASSET_ID);

        let ret = match asset {
            Ok(asset) => asset.id,
            Err(_) => -1
        };

        assert_eq!(ret, ASSET_ID);
    }
}