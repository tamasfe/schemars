mod util;

use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    JsonSchema,
};
use serde::Serialize;
use util::*;

fn custom(_gen: SchemaGenerator) -> Schema {
    let mut schema = SchemaObject::default();
    schema.metadata().title = Some("my custom schema".to_owned());
    schema.into()
}

#[derive(Debug, JsonSchema, Serialize)]
pub struct MyStruct {
    #[schemars(schema_with = "custom")]
    custom_property: String,
    derived_property: String,
    custom_struct: CustomSchemaStruct,
    tuple: TupleStruct,
    new_type: NewTypeStruct,
}

#[derive(Debug, JsonSchema, Serialize)]
pub struct TupleStruct(#[schemars(schema_with = "custom")] String, String);

#[derive(Debug, JsonSchema, Serialize)]
pub struct NewTypeStruct(#[schemars(schema_with = "custom")] String);

#[derive(Debug, JsonSchema, Serialize)]
#[schemars(schema_with = "custom")]
pub struct CustomSchemaStruct;

#[test]
fn custom_schema_for_struct() -> TestResult {
    test_default_generated_schema::<MyStruct>("custom_schema_for_struct")
}

#[derive(Debug, JsonSchema, Serialize)]
pub enum MyEnum {
    #[schemars(schema_with = "custom")]
    CustomUnit,
    DerivedUnit,
    Tuple(#[schemars(schema_with = "custom")] String, String),
    NewType(#[schemars(schema_with = "custom")] String),
    Struct {
        #[schemars(schema_with = "custom")]
        custom_property: String,
        derived_property: String,
        custom_enum: CustomSchemaEnum,
    },
}

#[derive(Debug, JsonSchema, Serialize)]
#[schemars(schema_with = "custom")]
pub enum CustomSchemaEnum {}

#[test]
fn custom_schema_for_enum() -> TestResult {
    test_default_generated_schema::<MyEnum>("custom_schema_for_enum")
}
