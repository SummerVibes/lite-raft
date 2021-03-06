mod service;
mod server;

use std::sync::Arc;
use std::collections::HashMap;

pub struct Server {
    pub service_map: Arc<HashMap<String,Service>>
}

pub struct Service{

}
#[cfg(test)]
mod tests {
    #[test]
    fn test_server(){

    }
}