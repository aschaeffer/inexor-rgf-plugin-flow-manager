use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use inexor_rgf_core_plugins::PluginContext;

use crate::config::FlowLocation;

#[async_trait]
pub trait FlowManager: Send + Sync {
    fn init(&self);

    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn load_config(&self);

    fn load_flows(&self, flow_locations: Vec<FlowLocation>);

    fn load_flows_by_path_recursively(&self, path: &Path);

    fn load_flow(&self, path: &Path);
}
