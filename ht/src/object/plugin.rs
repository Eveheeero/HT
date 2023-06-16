use super::DataType;
use intmap::IntMap;
use serde::{Deserialize, Serialize};

pub struct PluginData(pub IntMap<DataType>);

#[repr(u64)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum PluginDataCode {
    Name = 0,
    Description = 1,
    PluginEntry = 2,
    UnParker = 3,
    Authors = 5763822220893348879,
}
