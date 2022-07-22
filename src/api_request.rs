pub fn get<T>(
    url: &String,
) -> Option<T>
where
    for<'a> T: serde::de::Deserialize<'a>
{
    let res = ureq::get(url).call();

    match res {
        Ok(req) => {
            let json_pre: Result<T, _> = req.into_json();

            match json_pre {
                Ok(ret) => Some(ret),
                Err(_) => None
            }
        },

        Err(_) => None
    }
}
