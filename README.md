# actix-web-jsonschema

[![Latest Version](https://img.shields.io/crates/v/actix-web-jsonschema.svg?color=green&style=flat-square)](https://crates.io/crates/actix-web-jsonschema)
[![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
[![Documentation](https://docs.rs/actix-web-jsonschema/badge.svg)](https://docs.rs/actix-web-jsonschema)

This crate is a Rust library for providing validation mechanism
to [actix-web](https://github.com/actix/actix-web) with [jsonschema](https://github.com/Stranger6667/jsonschema-rs) crate.

More information about this crate can be found in the [crate documentation](https://docs.rs/actix-web-jsonschema).

### Installation
This crate works with Cargo and can be found on [crates.io](https://crates.io/crates/actix-web-jsonschema) with a Cargo.toml like:

```toml
[dependencies]
actix-web = { version = "4", features = ["macros"] }
actix-web-jsonschema = { version = "1", features = ["validator"] }
serde = { version = "1", features = ["derive"] }
schemars = { version = "0.8" }
validator = { version = "0.16", features = ["derive"] }
```

### Supported extractors
| actix_web                                                                                 | actix_web_jsonschema                                                                                              |
| :---------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| [actix_web::web::Path](https://docs.rs/actix-web/latest/actix_web/web/struct.Path.html)   | [actix_web_jsonschema::Path](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Path.html)   |
| [actix_web::web::Query](https://docs.rs/actix-web/latest/actix_web/web/struct.Query.html) | [actix_web_jsonschema::Query](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Query.html) |
| [actix_web::web::Form](https://docs.rs/actix-web/latest/actix_web/web/struct.Form.html)   | [actix_web_jsonschema::Form](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Form.html)   |
| [actix_web::web::Json](https://docs.rs/actix-web/latest/actix_web/web/struct.Json.html)   | [actix_web_jsonschema::Json](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Json.html)   |

### Example

```rust
use actix_web::{web, App};
use serde::Deserialize;
use schemars::JsonSchema;
use validator::Validate;
use actix_web_jsonschema::Query;

#[derive(Deserialize, JsonSchema, Validate)]
struct Request {
    #[validate(length(min = 1, max = 20))]
    name: String,
}

async fn index(Query(Request{ name }): Query<Request>) -> String {
    format!("Hello, {name}!")
}

fn main() {
    let app = App::new().service(
        web::resource("/hello").route(web::get().to(index))); // <- use `Query` extractor
}
```


License: MIT
