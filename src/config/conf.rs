use crate::ui::EventMap;
use gtk::gdk::RGBA;
use gtk4_layer_shell::{Edge, Layer};
use std::collections::HashMap;

pub type GroupConfigMap = HashMap<String, GroupConfig>;
pub type GroupConfig = Vec<Config>;

#[derive(Debug, Clone)]
pub enum MonitorSpecifier {
    ID(usize),
    Name(String),
}

pub struct Config {
    pub edge: Edge,
    pub position: Option<Edge>,
    pub layer: Layer,
    pub size: (f64, f64),
    pub rel_height: f64,
    pub event_map: Option<EventMap>,
    pub color: RGBA,
    pub transition_duration: u64,
    pub frame_rate: u64,
    pub extra_trigger_size: f64,
    pub monitor: MonitorSpecifier,
    // pub margins: Option<Vec<(Edge, i32)>>,
    pub margins: Vec<(Edge, i32)>,
}
#[derive(Debug)]
struct Test {
    edge: Edge,
    position: Option<Edge>,
    layer: Layer,
    size: (f64, f64),
    rel_height: f64,
    color: RGBA,
    transition_duration: u64,
    frame_rate: u64,
    extra_trigger_size: f64,
    monitor: MonitorSpecifier,
    margins: Vec<(Edge, i32)>,
}
impl Config {
    pub fn debug(&self) -> String {
        format!(
            "{:#?}",
            Test {
                edge: self.edge,
                position: self.position,
                layer: self.layer,
                size: self.size,
                color: self.color,
                transition_duration: self.transition_duration,
                frame_rate: self.frame_rate,
                extra_trigger_size: self.extra_trigger_size,
                monitor: self.monitor.clone(),
                margins: self.margins.clone(),
                rel_height: self.rel_height
            }
        )
    }
}
impl Drop for Config {
    fn drop(&mut self) {
        println!("dropping config")
    }
}