#![allow(non_snake_case)]

use yaserde_derive::{YaDeserialize, YaSerialize};

use super::ssc::*;

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct SystemStructureDescription {
    #[yaserde(attribute)]
    pub version: String,
    #[yaserde(attribute)]
    pub name: String,

    // ABaseElement
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub description: Option<String>,

    //ATOPLevelMetaData
    #[yaserde(attribute)]
    pub author: Option<String>,
    #[yaserde(attribute)]
    pub fileversion: Option<String>,
    #[yaserde(attribute)]
    pub copyright: Option<String>,
    #[yaserde(attribute)]
    pub license: Option<String>,
    #[yaserde(attribute)]
    pub generationTool: Option<String>,
    #[yaserde(attribute)]
    pub generationDateAndTime: Option<String>,

    #[yaserde(prefix = "ssd")]
    pub System: TSystem,
    #[yaserde(prefix = "ssc")]
    pub Enumerations: Option<TEnumerations>,
    #[yaserde(prefix = "ssc")]
    pub Units: Option<TUnits>,
    #[yaserde(prefix = "ssd")]
    pub DefaultExperiment: Option<TDefaultExperiment>,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TSystem {
    //TElemet
    // - ABaseElement
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub description: Option<String>,
    //TElemet cont
    #[yaserde(attribute)]
    pub name: Option<String>,
    #[yaserde(prefix = "ssd")]
    pub Connectors: Vec<TConnectors>,
    #[yaserde(prefix = "ssd")]
    pub ElementGeometry: Vec<ElementGeometry>,
    #[yaserde(prefix = "ssd")]
    pub TParameterBindings: Vec<TParameterBindings>,

    // TSystem cont
    #[yaserde(prefix = "ssd")]
    pub Elements: Option<Elements>,
    #[yaserde(prefix = "ssd")]
    pub Connections: Option<Connections>,
    #[yaserde(prefix = "ssd")]
    pub SignalDictionaries: Option<TSignalDictionaries>,
    #[yaserde(prefix = "ssd")]
    pub SystemGeometry: Option<SystemGeometry>,
    #[yaserde(prefix = "ssd")]
    pub GraphicalElements: Option<GraphicalElements>,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct Elements {
    #[yaserde(rename = "Component", prefix = "ssd")]
    pub Components: Vec<TComponent>,
    #[yaserde(rename = "SignalDictionaryReference", prefix = "ssd")]
    pub SignalDictionaryReferences: Vec<TSignalDictionaryReference>,
    #[yaserde(rename = "System", prefix = "ssd")]
    pub Systems: Vec<TSystem>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TComponent {
    //TElemet
    // - ABaseElement
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub description: Option<String>,
    //TElemet cont
    #[yaserde(attribute)]
    pub name: Option<String>,
    #[yaserde(prefix = "ssd")]
    pub Connectors: Vec<TConnectors>,
    #[yaserde(prefix = "ssd")]
    pub ElementGeometry: Vec<ElementGeometry>,
    #[yaserde(prefix = "ssd")]
    pub TParameterBindings: Vec<TParameterBindings>,

    #[yaserde(attribute, rename = "type")]
    pub r#type: Option<ComponentKind>,
    #[yaserde(attribute)]
    pub source: String,
    #[yaserde(attribute)]
    pub implementation: Option<String>,

    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

#[derive(Clone, Debug, PartialEq, YaSerialize)]
pub enum ComponentKind {
    XFmuSharedlibrary(String),
    XSspDefinition(String),
    XSspPackage(String),
}

///////////////////////////////////////////////////////////////////////////
// Connectors
///////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TConnectors {
    #[yaserde(rename = "Connector", prefix = "ssd")]
    pub list: Vec<Connector>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct Connector {
    // ABaseElement
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub description: Option<String>,

    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub kind: String,

    //ssc:GTypeChoice
    #[yaserde(prefix = "ssc")]
    pub Real: Option<GTypeReal>,
    #[yaserde(prefix = "ssc")]
    pub Integer: Option<GTypeInteger>,
    #[yaserde(prefix = "ssc")]
    pub Boolean: Option<GTypeBoolean>,
    #[yaserde(prefix = "ssc")]
    pub String: Option<GTypeString>,
    #[yaserde(prefix = "ssc")]
    pub Enumeration: Option<GTypeEnumeration>,
    #[yaserde(prefix = "ssc")]
    pub Binary: Option<GTypeBinary>,

    #[yaserde(prefix = "ssd")]
    pub ConnectorGeometry: Option<ConnectorGeometry>,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

///////////////////////////////////////////////////////////////////////////
// Connections
///////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct Connections {
    #[yaserde(rename = "Connection", prefix = "ssd")]
    pub list: Vec<Connection>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct Connection {
    #[yaserde(attribute)]
    pub startElement: Option<String>,
    #[yaserde(attribute)]
    pub startConnector: String,
    #[yaserde(attribute)]
    pub endElement: Option<String>,
    #[yaserde(attribute)]
    pub endConnector: String,
    #[yaserde(attribute)]
    pub suppressUnitConversion: Option<bool>,
    #[yaserde(prefix = "ssd")]
    pub LinearTransformation: Option<LinearTransformation>,
    #[yaserde(prefix = "ssd")]
    pub BooleanMappingTransformation: Option<BooleanMappingTransformation>,
    #[yaserde(prefix = "ssd")]
    pub IntegerMappingTransformation: Option<IntegerMappingTransformation>,
    #[yaserde(prefix = "ssd")]
    pub EnumerationMappingTransformation: Option<EnumerationMappingTransformation>,
    #[yaserde(prefix = "ssd")]
    pub ConnectionGeometry: Option<ConnectionGeometry>,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

///////////////////////////////////////////////////////////////////////////
// Geometry
///////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct SystemGeometry {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct ConnectorGeometry {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct ElementGeometry {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct ConnectionGeometry {}

///////////////////////////////////////////////////////////////////////////
// Other
///////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TSignalDictionaryReference {
    #[yaserde(attribute)]
    pub dictionary: String,
    #[yaserde(prefix = "ssd")]
    pub Connectors: Vec<TConnectors>,
    #[yaserde(prefix = "ssd")]
    pub ElementGeometry: Vec<ElementGeometry>,
    #[yaserde(prefix = "ssd")]
    pub TParameterBindings: Vec<TParameterBindings>,
    #[yaserde(prefix = "ssc")]
    pub Annotations: Option<TAnnotations>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TSignalDictionaries {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TParameterBindings {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct GraphicalElements {}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    prefix = "ssd",
    namespace = "ssd: http://ssp-standard.org/SSP1/SystemStructureDescription"
)]
pub struct TDefaultExperiment {}
