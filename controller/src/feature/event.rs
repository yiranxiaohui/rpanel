use std::sync::OnceLock;
use tokio::sync::broadcast;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct WebEvent {
    pub event_type: String, 
    pub payload: String,
}

pub static EVENT_BUS: OnceLock<broadcast::Sender<WebEvent>> = OnceLock::new();

pub fn get_event_bus() -> &'static broadcast::Sender<WebEvent> {
    EVENT_BUS.get_or_init(|| {
        let (tx, _) = broadcast::channel(100);
        tx
    })
}

pub fn send_event(event_type: &str, payload: String) {
    let bus = get_event_bus();
    let event = WebEvent {
        event_type: event_type.to_string(),
        payload,
    };
    // Ignore error if no listeners
    let _ = bus.send(event);
}
