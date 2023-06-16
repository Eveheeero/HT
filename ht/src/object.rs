mod data;
mod plugin;

pub use data::*;
use half::f16;
use intmap::IntMap;
use num_bigint::{BigInt, BigUint};
pub use plugin::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Inner data type
#[derive(Clone, Serialize, Deserialize)]
pub enum DataType {
    None,
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F16(f16),
    F32(f32),
    F64(f64),
    Vector(Vec<DataType>),
    Map(MapData),
    Set(SetData),
    BigUint(BigUint),
    BigInt(BigInt),
    Uuid(Uuid),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MapData {
    Dynamic(HashMap<DataTypeWithoutCollection, DataType>),
    String(HashMap<String, DataType>),
    Bool(HashMap<bool, DataType>),
    U8(HashMap<u8, DataType>),
    U16(HashMap<u16, DataType>),
    U32(HashMap<u32, DataType>),
    U64(IntMap<DataType>),
    U128(HashMap<u128, DataType>),
    I8(HashMap<i8, DataType>),
    I16(HashMap<i16, DataType>),
    I32(HashMap<i32, DataType>),
    I64(HashMap<i64, DataType>),
    I128(HashMap<i128, DataType>),
    BigUint(HashMap<BigUint, DataType>),
    BigInt(HashMap<BigInt, DataType>),
    Uuid(HashMap<Uuid, DataType>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SetData {
    Dynamic(HashSet<DataTypeWithoutCollection>),
    String(HashSet<String>),
    Bool(HashSet<bool>),
    U8(HashSet<u8>),
    U16(HashSet<u16>),
    U32(HashSet<u32>),
    U64(HashSet<u64>),
    U128(HashSet<u128>),
    I8(HashSet<i8>),
    I16(HashSet<i16>),
    I32(HashSet<i32>),
    I64(HashSet<i64>),
    I128(HashSet<i128>),
    BigUint(HashSet<BigUint>),
    BigInt(HashSet<BigInt>),
    Uuid(HashSet<Uuid>),
}

/// Data Type for key for Map and Set
#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum DataTypeWithoutCollection {
    None,
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    BigUint(BigUint),
    BigInt(BigInt),
    Uuid(Uuid),
}

/// 내부 데이터를 수정하면 get_modified_num 값이 바뀝니다.
///
/// ```ignore
/// let cell = DataCell::new(DataType::String("hello world".to_string()));
/// /* ... */
/// let hash = cell.get_modified_number();
/// let data = cell.get();
/// /* Do SomeThing With Data... */
/// if hash == cell.get_modified_number() {
///   cell.set(result_of_SomeThing);
/// } else {
///   /* Try again with new value */
/// }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct DataCell(DataType, u64);

impl DataCell {
    pub fn new(data: DataType) -> Self {
        Self(data, fastrand::u64(..))
    }
    pub fn get(&self) -> &DataType {
        &self.0
    }
    pub fn get_modified_number(&self) -> u64 {
        self.1
    }
    pub fn set(&mut self, data: DataType) {
        self.0 = data;
        self.1 = fastrand::u64(..);
    }
}
