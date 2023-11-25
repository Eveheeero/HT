use super::DataCell;
use intmap::IntMap;
use serde::{Deserialize, Serialize};

/// 실제 게임 데이터 객체에 대한 데이터들
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Data(IntMap<DataCell>);

#[repr(u64)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum DataCode {
    Name = 0,
    Hp = 1,
    Description = 2,
}

impl Data {
    #[inline(always)]
    pub fn insert(&mut self, key: DataCode, value: DataCell) -> Option<DataCell> {
        self.0.insert(key as u64, value)
    }
    #[inline(always)]
    pub fn get(&self, key: DataCode) -> Option<&DataCell> {
        self.0.get(key as u64)
    }
    #[inline(always)]
    pub fn get_mut(&mut self, key: DataCode) -> Option<&mut DataCell> {
        self.0.get_mut(key as u64)
    }
    #[inline(always)]
    pub fn remove(&mut self, key: DataCode) -> Option<DataCell> {
        self.0.remove(key as u64)
    }
}
