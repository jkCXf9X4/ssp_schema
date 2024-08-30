#![allow(non_snake_case)]
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct TEnumerations {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct TUnits {
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub description: Option<String>,
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(prefix = "ssc")]
    pub BaseUnit: BaseUnit,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct BaseUnit {
    #[yaserde(attribute)]
    pub kg: Option<i32>,
    #[yaserde(attribute)]
    pub m: Option<i32>,
    #[yaserde(attribute)]
    pub s: Option<i32>,
    #[yaserde(attribute)]
    pub a: Option<i32>,
    #[yaserde(attribute)]
    pub k: Option<i32>,
    #[yaserde(attribute)]
    pub mol: Option<i32>,
    #[yaserde(attribute)]
    pub cd: Option<i32>,
    #[yaserde(attribute)]
    pub rad: Option<i32>,
    #[yaserde(attribute)]
    pub factor: Option<f64>,
    #[yaserde(attribute)]
    pub offset: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct TAnnotations {
    #[yaserde(rename = "Annotation", prefix = "ssc")]
    pub list: Vec<Annotation>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeReal {
    #[yaserde(attribute)]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeInteger {
    // No elements
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeBoolean {
    // No elements
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeString {
    // No elements
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeEnumeration {
    #[yaserde(attribute)]
    pub name: Option<String>, // Should not be optional but some tools dont include the enum correctly 
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct GTypeBinary {
    #[yaserde(rename = "mime-type", attribute)]
    pub mime_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct LinearTransformation {
    #[yaserde(attribute)]
    pub factor: Option<f64>,
    #[yaserde(attribute)]
    pub offset: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct BooleanMappingTransformation {
    #[yaserde(rename = "MapEntry", prefix = "ssd")]
    pub list: Vec<BooleanMapEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct BooleanMapEntry {
    #[yaserde(attribute)]
    pub source: bool,
    #[yaserde(attribute)]
    pub target: bool,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct IntegerMappingTransformation {
    #[yaserde(rename = "MapEntry", prefix = "ssd")]
    pub list: Vec<BooleanMapEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct IntegerMapEntry {
    #[yaserde(attribute)]
    pub source: i32,
    #[yaserde(attribute)]
    pub target: i32,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct EnumerationMappingTransformation {
    #[yaserde(rename = "MapEntry", prefix = "ssd")]
    pub list: Vec<EnumerationMapEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct EnumerationMapEntry {
    #[yaserde(attribute)]
    pub source: String,
    #[yaserde(attribute)]
    pub target: String,
}

// TODO: Write custom parser to extract the any part of annotations....

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssc",
    namespace = "ssc: http://ssp-standard.org/SSP1/SystemStructureCommon"
)]
pub struct Annotation {
    #[yaserde(attribute = "type")]
    pub r#type: Option<String>,
    pub any: Option<String>,
}
