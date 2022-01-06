use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::model::Flow;
use async_trait::async_trait;
use log::{debug, error};
use waiter_di::*;

use crate::api::FlowManager;
use crate::config::{FlowLocation, FlowLocations};
use crate::plugins::PluginContext;

const CONFIG_PATH: &str = "./config/flow_locations.toml";

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[component]
pub struct FlowManagerImpl {
    context: PluginContextContainer,
}

impl FlowManagerImpl {}

#[async_trait]
#[provides]
impl FlowManager for FlowManagerImpl {
    fn init(&self) {
        self.load_config();
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn load_config(&self) {
        let toml_config = std::fs::read_to_string(Path::new(CONFIG_PATH));
        match toml_config {
            Ok(toml_string) => {
                let flow_locations: Result<FlowLocations, _> = toml::from_str(&toml_string);
                match flow_locations {
                    Ok(flow_locations) => {
                        self.load_flows(flow_locations.location);
                    }
                    Err(_) => {
                        error!("Failed to load flow manager configuration from {}: Invalid TOML:", CONFIG_PATH);
                    }
                }
            }
            Err(_) => {
                error!("Failed to load flow manager configuration from {}", CONFIG_PATH);
            }
        }
    }

    fn load_flows(&self, flow_locations: Vec<FlowLocation>) {
        for flow_location in flow_locations.into_iter() {
            if flow_location.active {
                let path = Path::new(flow_location.path.as_str());
                self.load_flows_by_path_recursively(path);
            }
        }
    }

    fn load_flows_by_path_recursively(&self, path: &Path) {
        if !path.is_dir() {
            error!("Not a directory: {:?}", path);
            return;
        }
        let result = fs::read_dir(path);
        match fs::read_dir(path) {
            Ok(dir) => {
                for entry in dir.into_iter() {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                self.load_flows_by_path_recursively(&path);
                            } else {
                                self.load_flow(&path);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn load_flow(&self, path: &Path) {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let result: Result<Flow, serde_json::Error> = serde_json::from_reader(reader);
                match result {
                    Ok(flow) => {
                        let reader = self.context.0.read().unwrap();
                        let flow_manager = reader.as_ref().unwrap().get_flow_manager().clone();
                        match flow_manager.create(flow) {
                            Ok(flow) => {
                                debug!("Successfully loaded flow {:?} (id: {})", path, flow.id);
                            }
                            Err(err) => {
                                error!("Failed to load flow {:?}: {:?}", path, err);
                            }
                        }
                    }
                    _ => {
                        error!("Cannot parse flow {:?}", path);
                    }
                }
            }
            _ => {
                error!("Failed to load flow from {:?}", path);
            }
        }
    }
}
