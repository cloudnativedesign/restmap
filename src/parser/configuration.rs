use serde_derive::{Serialize, Deserialize};
use std::path::Path;
use std::collections::HashMap;
// Holds the temporary hard-coded configuration for the first version of the schema
// Uses the typepag crate to automatically create an enum definition for the types of 

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    version: String,
    kind: ConfigKind,
    config: Box<dyn Config>,
}
// Configuration types for the supported template "kinds"
#[derive(Serialize, Deserialize)]
enum ConfigKind {
    Job,
    Endpoint,
    Resolver,
}
#[derive(Serialize, Deserialize)]
struct JobConfig {
    endpoints: Vec<Box<dyn Endpoint>>,
    params: Option<ParamReference>,
}
#[typetag::serde]
impl Config for JobConfig {}

#[derive(Serialize, Deserialize)]
struct BaseEndpointConfig {
}
#[typetag::serde]
impl Endpoint for BaseEndpointConfig {}

#[derive(Serialize, Deserialize)]
struct RelativeEndpointConfig {
    
}
#[typetag::serde]
impl Endpoint for RelativeEndpointConfig {}

#[derive(Serialize, Deserialize)]
struct SQLResolver {
    name: String,
    connectionstring: String,
    table: TableConfig
}
#[derive(Serialize, Deserialize)]
struct TableConfig {
    name: String,
    select: String // A SQL statement to select the column to be returned as the array
}
#[typetag::serde]
impl Resolver for SQLResolver {}

#[derive(Serialize, Deserialize)]
struct FileResolver {
    name: String,
    path: String,

}
#[typetag::serde]
impl Resolver for FileResolver {}


#[derive(Serialize, Deserialize)]
struct ParamReference {
    param: String,
    default: Option<String>,
}
#[typetag::serde]
impl Config for ParamReference {}

#[derive(Serialize, Deserialize)]
struct ParameterConfig {
    name: String,
    datatype: String,
    loader: String
}
#[typetag::serde]
impl Config for ParameterConfig{}

#[derive(Serialize, Deserialize)]
struct LoaderParamConfig {
    

}
#[typetag::serde]
impl Config for LoaderParamConfig{}

#[derive(Serialize, Deserialize)]
struct EndpointParamConfig {
    

}
#[typetag::serde]
impl Config for EndpointParamConfig{}

// Defines the base traits to generalize the Deserialization
#[typetag::serde(tag="config")]
pub trait Config {}
#[typetag::serde(tag="endpoint")]
pub trait Endpoint {}
#[typetag::serde(tag="resolver")]
pub trait Resolver {}
