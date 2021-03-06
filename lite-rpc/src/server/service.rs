use crate::server::Service;
use std::collections::HashMap;
use std::sync::Arc;

type ServiceMap = Arc<HashMap<String,Service>>;