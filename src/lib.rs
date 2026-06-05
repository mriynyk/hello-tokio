use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod connection;
pub mod process_socket;

pub type Db = Arc<Mutex<HashMap<String, Bytes>>>;
