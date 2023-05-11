/// specification https://swagger.io/specification/v2/
use serde_json::{Deserialize, Serialize, Value};

// basic definitions enums
#[derive(Debug, Serialize, Deserialize)]
enum ParameterIn {
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
}

#[derive(Debug, Serialize, Deserialize)]
enum SchemaType {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsObject {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionsObject {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwaggerObject {
    swagger: String,
    paths: HashMap<String, PathItemObject>,
    defsinitions: Option<HashMap<String, SchemaObject>>,
    // after this is not supported now.
    securityDefinitions: Option<SecurityDefinitionsObject>,
    security: Option<Vec<SecurityRequirementObject>>,
    tags: Option<Vec<TagObject>>,
    externalDocs: Option<ExternalDocumentationObject>,
    parameters: Option<ParametersDefinitionsObject>,
    responses: Option<ResponsesDefinitionsObject>,
    basePath: Option<String>,
    host: Option<String>,
    info: InfoObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathItemObject {
    get: Option<OperationObject>,
    put: Option<OperationObject>,
    post: Option<OperationObject>,
    delete: Option<OperationObject>,
    options: Option<OperationObject>,
    head: Option<OperationObject>,
    patch: Option<OperationObject>,
    /// after this is not supported now.
    parameters: Option<Vec<ParameterObject>>,
    #[serde(rename = "$ref")]
    ref_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationObject {
    description: Option<String>,
    operationId: Option<String>,
    parameters: Option<Vec<ParameterObject>>,
    responses: Option<HashMap<String, ResponseObject>>,
    schemes: Option<Vec<String>>,
    deprecated: Option<bool>,
    /// after this is not supported now.
    produces: Option<Vec<String>>,
    consumes: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    externalDocs: Option<ExternalDocumentationObject>,
    summary: Option<String>,
    security: Option<Vec<SecurityRequirementObject>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterObject {
    #[serde(rename = "type")]
    type_: Option<SchemaType>,
    #[serde(rename = "in")]
    in_: ParameterIn,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    /// merge ref here.
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    name: String,
    required: Option<bool>,
    schema: Option<SchemaObject>,
}

/// Items Object
/// A limited subset of JSON-Schema's items object. It is used by parameter definitions that are not located in "body".
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsObject {
    #[serde(rename = "type")]
    type_: SchemaType,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    description: Option<String>,
    items: Option<Box<ItemsObject>>,
}

/// unsported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsesObject {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObject {
    /// merge ref here.
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    description: Option<String>,
    schema: Option<SchemaObject>,
    /// unsported now.
    headers: Option<HashMap<String, HeaderObject>>,
    /// unsported now.
    examples: Option<HashMap<String, ExampleObject>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceObject {
    #[serde(rename = "$ref")]
    ref_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaObject {
    #[serde(rename = "type")]
    type_: Option<SchemaType>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    title: Option<String>,
    description: Option<String>,
    items: Option<Box<SchemaObject>>,
    properties: Option<HashMap<String, SchemaObject>>,
    schema: Option<Box<SchemaObject>>,
    required: Option<Vec<String>>,
    allOf: Option<Vec<SchemaObject>>,
    additionalProperties: Option<Box<SchemaObject>>,
}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct XMLObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParametersDefinitionsObject {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsesDefinitionsObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityDefinitionsObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritySchemeObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityRequirementObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadersObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct TagObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocumentationObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactObject {}

/// not supported now.
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseObject {}

pub trait printable {
    fn to_dts(&self) -> &str;
    fn cast(&self, v: &Value) -> Result<Self>;
}
