use std::collections::HashMap;

use crate::model::reference::Value;

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Finalize, Trace)]
pub struct Scope(HashMap<String, Value>);
