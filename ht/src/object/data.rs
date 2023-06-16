use super::DataCell;
use intmap::IntMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data(pub IntMap<DataCell>);

#[repr(u64)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum DataCode {
    Name = 0,
    Hp = 1,
    Description = 2,
}
