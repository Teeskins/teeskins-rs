pub fn get_request<T>(
    url: &String,
) -> Result<T, ureq::Error>
where
    for<'a> T: serde::de::Deserialize<'a>
{
    let res: serde_json::Value = ureq::get(url)
        .call()?
        .into_json()?;
    
    let ret = serde_json::from_value(res);
    Ok(ret.unwrap())    
}
