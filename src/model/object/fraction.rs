use super::Object;

use crate::model::reference::Value;

use gc::{Finalize, Trace};
use rug::Integer as BigInt;

#[derive(Trace, Finalize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fraction {
    #[unsafe_ignore_trace]
    pub numerator: BigInt,
    #[unsafe_ignore_trace]
    pub denominator: BigInt,
}

impl Object for Fraction {
    fn index(&self, index: super::table::Table) -> Value {
        todo!("FIXME <Fraction as Object>::index has not yet been implemented");
    }

    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <Fraction as Object>::get_field has not yet been implemented");
    }
}
