use std::{
    any::{type_name, TypeId},
    cell::RefCell,
    collections::HashMap,
};

use jsonschema::{output::BasicOutput, JSONSchema};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use serde::Serialize;
use serde_json::{Map, Value};

pub trait SchemaSerialize: Serialize + 'static {}
impl<T> SchemaSerialize for T where T: Serialize + 'static {}

#[cfg(not(feature = "validator"))]
mod deserialize {
    pub trait SchemaDeserialize:
        serde::de::DeserializeOwned + schemars::JsonSchema + 'static
    {
    }

    impl<T> SchemaDeserialize for T where T: serde::de::DeserializeOwned + schemars::JsonSchema + 'static
    {}
}

#[cfg(feature = "validator")]
mod deserialize {
    pub trait SchemaDeserialize:
        serde::de::DeserializeOwned + validator::Validate + schemars::JsonSchema + 'static
    {
    }

    impl<T> SchemaDeserialize for T where
        T: serde::de::DeserializeOwned + validator::Validate + schemars::JsonSchema + 'static
    {
    }
}

pub use deserialize::SchemaDeserialize;

thread_local! {
    static CONTEXT: RefCell<SchemaContext> = RefCell::new(SchemaContext::new());
}

pub(crate) struct SchemaContext {
    pub generator: SchemaGenerator,
    pub schemas: HashMap<TypeId, JSONSchema>,
}

impl SchemaContext {
    pub fn new() -> Self {
        Self {
            generator: SchemaSettings::draft07()
                .with(|settings| settings.inline_subschemas = true)
                .into_generator(),
            schemas: HashMap::default(),
        }
    }

    pub fn from_value<T>(value: Value) -> Result<T, crate::Error>
    where
        T: SchemaDeserialize,
    {
        CONTEXT.with(|ctx| {
            let ctx = &mut *ctx.borrow_mut();
            let schema = ctx.schemas.entry(TypeId::of::<T>()).or_insert_with(|| {
                jsonschema::JSONSchema::compile(
                    &serde_json::to_value(ctx.generator.root_schema_for::<T>()).unwrap(),
                )
                .unwrap_or_else(|err| {
                    tracing::error!(
                        %err,
                        type_name = type_name::<T>(),
                        "invalid JSON schema for type"
                    );
                    JSONSchema::compile(&Value::Object(Map::default())).unwrap()
                })
            });

            SchemaContext::validate(value, schema)
        })
    }

    fn validate<T>(
        value: serde_json::Value,
        schema: &jsonschema::JSONSchema,
    ) -> Result<T, crate::Error>
    where
        T: SchemaDeserialize,
    {
        if let BasicOutput::Invalid(err) = schema.apply(&value).basic() {
            Err(crate::Error::JsonSchema(err))?
        }

        let data: T = serde_json::from_value(value).map_err(crate::Error::SerdeJson)?;

        #[cfg(feature = "validator")]
        data.validate().map_err(crate::Error::Validator)?;

        Ok(data)
    }
}
