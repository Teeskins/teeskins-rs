# Teeskins API wrapper

## How to build and run ?

1. Install the dependencies 
   - `cargo`

2. Create the file `.env` in the repository source using the `.env_example`
    - Complete the fields

## Usage example

```rust
use teeskins_rs::api::TeeskinsApi;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    let host = env::var("HOST")
    .unwrap_or("https://localhost".to_string());
    
    let api = TeeskinsApi::new(
        "".to_string(),
        host
    );
    
    let asset = api.get_profile("nagi01".to_string());
        println!("{}", match asset {
            Some(v) =>  v.user.name,
            None => String::from("Name has not been found")
        }
    )
}
```

## Unitary tests
Run `cargo test`

The tests are using the following environment variables:
- `HOST`
- `DISCORD_TOKEN`
