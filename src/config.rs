use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowLocations {
    pub location: Vec<FlowLocation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowLocation {
    /// The name of the flow location.
    pub name: String,

    /// If false, the flow location will be ignored.
    #[serde(default = "default_true")]
    pub active: bool,

    /// The path to the flow location
    pub path: String,
}

fn default_true() -> bool {
    true
}
