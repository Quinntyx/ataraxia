pub mod float;
pub mod function;
pub mod primitive;
pub mod table;

use dyn_clone::DynClone;

use std::fmt::Debug;

pub trait Object: DynClone + Debug {}

dyn_clone::clone_trait_object!(Object);
