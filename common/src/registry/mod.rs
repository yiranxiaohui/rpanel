use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub type HandlerFn = fn() ;

pub struct HandlerEntry {
    pub method: &'static str,
    pub func: HandlerFn,
}

inventory::collect!(HandlerEntry);

pub static ROUTES: LazyLock<RwLock<HashMap<String, HandlerFn>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn init_routes() {
    for e in inventory::iter::<HandlerEntry> {
        let e: &HandlerEntry = e;
        ROUTES.write().await.insert(e.method.to_string(), e.func);
    }
}