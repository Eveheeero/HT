use super::DataType;
use intmap::IntMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PluginData(IntMap<DataType>);

#[repr(u64)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum PluginDataCode {
    Name = 0,
    Description = 1,
    PluginEntry = 2,
    UnParker = 3,
    Authors = 5763822220893348879,
}

impl PluginData {
    #[inline(always)]
    pub fn insert(&mut self, key: PluginDataCode, value: DataType) -> Option<DataType> {
        self.0.insert(key as u64, value)
    }
    #[inline(always)]
    pub fn get(&self, key: PluginDataCode) -> Option<&DataType> {
        self.0.get(key as u64)
    }
    #[inline(always)]
    pub fn get_mut(&mut self, key: PluginDataCode) -> Option<&mut DataType> {
        self.0.get_mut(key as u64)
    }
    #[inline(always)]
    pub fn remove(&mut self, key: PluginDataCode) -> Option<DataType> {
        self.0.remove(key as u64)
    }
}
