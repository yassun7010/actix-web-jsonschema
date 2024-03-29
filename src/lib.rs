//! [![Latest Version](https://img.shields.io/crates/v/actix-web-jsonschema.svg?color=green&style=flat-square)](https://crates.io/crates/actix-web-jsonschema)
//! [![Documentation](https://docs.rs/actix-web-jsonschema/badge.svg)](https://docs.rs/actix-web-jsonschema)
//! [![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
//!
//! This crate is a Rust library for providing validation mechanism
//! to [actix-web](https://github.com/actix/actix-web) with [jsonschema](https://github.com/Stranger6667/jsonschema-rs) crate.
//!
//! More information about this crate can be found in the [crate documentation](https://docs.rs/actix-web-jsonschema).
//!
//! ## Installation
//!
//! This crate works with Cargo and can be found on [crates.io](https://crates.io/crates/actix-web-jsonschema) with a Cargo.toml like:
//!
//! ```toml
//! [dependencies]
//! actix-web = { version = "4", features = ["macros"] }
//! actix-web-jsonschema = { version = "1", features = ["validator"] }
//! serde = { version = "1", features = ["derive"] }
//! schemars = { version = "0.8" }
//! validator = { version = "0.16", features = ["derive"] }
//! ```
//!
//! ## Feature Flags
//!
//! - `validator` - provides [validator](https://github.com/Keats/validator) validation.
//! - `qs_query` - provides [QsQuery](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.QsQuery.html) extractor.
//!
//! ## Supported extractors
//!
//! | actix_web                                                                                      | actix_web_jsonschema                                                                                                  |
//! | :--------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
//! | [actix_web::web::Path](https://docs.rs/actix-web/latest/actix_web/web/struct.Path.html)        | [actix_web_jsonschema::Path](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Path.html)       |
//! | [actix_web::web::Query](https://docs.rs/actix-web/latest/actix_web/web/struct.Query.html)      | [actix_web_jsonschema::Query](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Query.html)     |
//! | [actix_web::web::Form](https://docs.rs/actix-web/latest/actix_web/web/struct.Form.html)        | [actix_web_jsonschema::Form](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Form.html)       |
//! | [actix_web::web::Json](https://docs.rs/actix-web/latest/actix_web/web/struct.Json.html)        | [actix_web_jsonschema::Json](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.Json.html)       |
//! | [serde_qs::actix::QsQuery](https://docs.rs/serde_qs/latest/serde_qs/actix/struct.QsQuery.html) | [actix_web_jsonschema::QsQuery](https://docs.rs/actix-web-jsonschema/latest/actix_web_jsonschema/struct.QsQuery.html) |
//!
//! ## Example
//!
//! ```rust
//! use actix_web::{web, App};
//! use serde::Deserialize;
//! use schemars::JsonSchema;
//! use validator::Validate;
//! use actix_web_jsonschema::Query;
//!
//! #[derive(Deserialize, JsonSchema, Validate)]
//! struct Request {
//!     #[validate(length(min = 1, max = 20))]
//!     name: String,
//! }
//!
//! async fn index(Query(Request{ name }): Query<Request>) -> String {
//!     format!("Hello, {name}!")
//! }
//!
//! fn main() {
//!     let app = App::new().service(
//!         web::resource("/hello").route(web::get().to(index))); // <- use `Query` extractor
//! }
//! ```
//!

mod error;
mod macros;
mod schema;

use futures::FutureExt;

pub use error::Error;
use macros::jsonschema_extractor;

jsonschema_extractor! {
    #[doc = "Extract typed information from the request’s path."]
    #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
    pub struct Path<T>(pub T);
}

jsonschema_extractor! {
    #[doc = "Extract and validate typed information from the request’s query."]
    #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
    pub struct Query<T>(pub T);
}

jsonschema_extractor! {
    #[doc = "Form can be used for extracting typed information and validation from request’s form data."]
    #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
    pub struct Form<T>(pub T);
}

jsonschema_extractor! {
    #[doc = "Json can be used for exstracting typed information and validation from request’s payload."]
    #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest, Responder)]
    pub struct Json<T>(pub T);
}

#[cfg(feature = "qs_query")]
mod qs_query {
    use super::*;

    mod actix_web {
        pub use actix_web::*;

        pub mod web {
            pub use serde_qs::actix::QsQuery;
        }
    }

    jsonschema_extractor! {
        #[doc = "Extract and validate typed information from the request’s query ([serde_qs](https://crates.io/crates/serde_qs) based)."]
        #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
        pub struct QsQuery<T>(pub T);
    }
}
#[cfg(feature = "qs_query")]
pub use qs_query::QsQuery;

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::{
        body::to_bytes, dev::ServiceResponse, http::header::ContentType, test, web, App,
    };
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    async fn json_body(
        response: ServiceResponse,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = to_bytes(response.into_body()).await?;
        let value = serde_json::from_str::<serde_json::Value>(std::str::from_utf8(&body)?)?;

        Ok(value)
    }

    #[cfg(not(feature = "validator"))]
    mod default_tests {
        use super::*;

        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        struct Request {
            name: String,
        }

        #[derive(Debug, Serialize, JsonSchema)]
        struct Response {
            name: String,
        }

        async fn index(
            crate::Json(Request { name }): crate::Json<Request>,
        ) -> crate::Json<Response> {
            crate::Json(Response { name })
        }

        #[actix_web::test]
        async fn test_request_ok() {
            let app = test::init_service(App::new().route("/", web::get().to(index))).await;
            let request = test::TestRequest::default()
                .insert_header(ContentType::json())
                .set_json(Request {
                    name: "taro".to_string(),
                })
                .to_request();
            let response = test::call_service(&app, request).await;

            assert!(response.status().is_success());
        }

        #[actix_web::test]
        async fn test_required_key_err() -> Result<(), Box<dyn std::error::Error>> {
            let app = test::init_service(App::new().route("/", web::get().to(index))).await;
            let request = test::TestRequest::default()
                .insert_header(ContentType::json())
                .set_json(json!({}))
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(
                json_body(response).await?,
                json!([
                    {
                        "error": "\"name\" is a required property",
                        "instanceLocation": "",
                        "keywordLocation": "/required"
                    }
                ])
            );

            Ok(())
        }

        #[actix_web::test]
        async fn test_wrong_type_err() -> Result<(), Box<dyn std::error::Error>> {
            let app = test::init_service(App::new().route("/", web::get().to(index))).await;
            let request = test::TestRequest::default()
                .insert_header(ContentType::json())
                .set_json(json!({"name": 0}))
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(
                json_body(response).await?,
                json!([
                    {
                        "error": "0 is not of type \"string\"",
                        "instanceLocation": "/name",
                        "keywordLocation": "/properties/name/type"
                    }
                ])
            );

            Ok(())
        }
    }

    #[cfg(feature = "validator")]
    mod validator_tests {
        use super::*;
        use validator::Validate;

        #[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
        struct Request {
            #[validate(length(min = 1, max = 5))]
            name: String,
        }

        #[derive(Debug, Serialize, JsonSchema)]
        struct Response {
            name: String,
        }

        async fn index(
            crate::Json(Request { name }): crate::Json<Request>,
        ) -> crate::Json<Response> {
            crate::Json(Response { name })
        }

        #[actix_web::test]
        async fn test_request_ok() {
            let app = test::init_service(App::new().route("/", web::get().to(index))).await;
            let request = test::TestRequest::default()
                .insert_header(ContentType::json())
                .set_json(Request {
                    name: "taro".to_string(),
                })
                .to_request();

            let response = test::call_service(&app, request).await;

            assert!(response.status().is_success());
        }

        #[actix_web::test]
        async fn test_validation_error() -> Result<(), Box<dyn std::error::Error>> {
            let app = test::init_service(App::new().route("/", web::get().to(index))).await;
            let request = test::TestRequest::default()
                .insert_header(ContentType::json())
                .set_json(Request {
                    name: "kojiro".to_string(),
                })
                .to_request();

            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(
                json_body(response).await?,
                json!([
                    {
                        "error": "\"kojiro\" is longer than 5 characters",
                        "instanceLocation": "/name",
                        "keywordLocation": "/properties/name/maxLength"
                    }
                ])
            );

            Ok(())
        }
    }
}
