use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;
use crate::verifier::Verifier;

pub const VERIFIER: Item<Verifier> = Item::new("VERIFIER");