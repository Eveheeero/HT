mod data;
mod plugin;
mod world;

pub use data::*;
use half::f16;
use intmap::IntMap;
use num_bigint::{BigInt, BigUint};
pub use plugin::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
pub use world::*;

/// 내부 데이터타입
/// HP일경우 match u8, u16등 사용, 그 외의경우는 unimplement로 처리
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Data {
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
    Vector(Vec<Data>),
    Map(MapData),
    Set(SetData),
    BigUint(BigUint),
    BigInt(BigInt),
    Uuid(Uuid),
}
impl Data {
    #[inline(always)]
    pub fn string(&self) -> Option<&String> {
        if let Data::String(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bool(&self) -> Option<&bool> {
        if let Data::Bool(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u8(&self) -> Option<&u8> {
        if let Data::U8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u16(&self) -> Option<&u16> {
        if let Data::U16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u32(&self) -> Option<&u32> {
        if let Data::U32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u64(&self) -> Option<&u64> {
        if let Data::U64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u128(&self) -> Option<&u128> {
        if let Data::U128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i8(&self) -> Option<&i8> {
        if let Data::I8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i16(&self) -> Option<&i16> {
        if let Data::I16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i32(&self) -> Option<&i32> {
        if let Data::I32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i64(&self) -> Option<&i64> {
        if let Data::I64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i128(&self) -> Option<&i128> {
        if let Data::I128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn f16(&self) -> Option<&f16> {
        if let Data::F16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn f32(&self) -> Option<&f32> {
        if let Data::F32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn f64(&self) -> Option<&f64> {
        if let Data::F64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn vector(&self) -> Option<&Vec<Data>> {
        if let Data::Vector(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn map(&self) -> Option<&MapData> {
        if let Data::Map(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn set(&self) -> Option<&SetData> {
        if let Data::Set(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn biguint(&self) -> Option<&BigUint> {
        if let Data::BigUint(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bigint(&self) -> Option<&BigInt> {
        if let Data::BigInt(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn uuid(&self) -> Option<&Uuid> {
        if let Data::Uuid(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MapData {
    Dynamic(HashMap<DataWithoutCollection, Data>),
    String(HashMap<String, Data>),
    Bool(HashMap<bool, Data>),
    U8(HashMap<u8, Data>),
    U16(HashMap<u16, Data>),
    U32(HashMap<u32, Data>),
    U64(IntMap<Data>),
    U128(HashMap<u128, Data>),
    I8(HashMap<i8, Data>),
    I16(HashMap<i16, Data>),
    I32(HashMap<i32, Data>),
    I64(HashMap<i64, Data>),
    I128(HashMap<i128, Data>),
    BigUint(HashMap<BigUint, Data>),
    BigInt(HashMap<BigInt, Data>),
    Uuid(HashMap<Uuid, Data>),
}
impl MapData {
    #[inline(always)]
    pub fn dynamic(&self) -> Option<&HashMap<DataWithoutCollection, Data>> {
        if let MapData::Dynamic(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn string(&self) -> Option<&HashMap<String, Data>> {
        if let MapData::String(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bool(&self) -> Option<&HashMap<bool, Data>> {
        if let MapData::Bool(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u8(&self) -> Option<&HashMap<u8, Data>> {
        if let MapData::U8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u16(&self) -> Option<&HashMap<u16, Data>> {
        if let MapData::U16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u32(&self) -> Option<&HashMap<u32, Data>> {
        if let MapData::U32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u64(&self) -> Option<&IntMap<Data>> {
        if let MapData::U64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u128(&self) -> Option<&HashMap<u128, Data>> {
        if let MapData::U128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i8(&self) -> Option<&HashMap<i8, Data>> {
        if let MapData::I8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i16(&self) -> Option<&HashMap<i16, Data>> {
        if let MapData::I16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i32(&self) -> Option<&HashMap<i32, Data>> {
        if let MapData::I32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i64(&self) -> Option<&HashMap<i64, Data>> {
        if let MapData::I64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i128(&self) -> Option<&HashMap<i128, Data>> {
        if let MapData::I128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn biguint(&self) -> Option<&HashMap<BigUint, Data>> {
        if let MapData::BigUint(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bigint(&self) -> Option<&HashMap<BigInt, Data>> {
        if let MapData::BigInt(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn uuid(&self) -> Option<&HashMap<Uuid, Data>> {
        if let MapData::Uuid(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SetData {
    Dynamic(HashSet<DataWithoutCollection>),
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
impl SetData {
    #[inline(always)]
    pub fn dynamic(&self) -> Option<&HashSet<DataWithoutCollection>> {
        if let SetData::Dynamic(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn string(&self) -> Option<&HashSet<String>> {
        if let SetData::String(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bool(&self) -> Option<&HashSet<bool>> {
        if let SetData::Bool(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u8(&self) -> Option<&HashSet<u8>> {
        if let SetData::U8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u16(&self) -> Option<&HashSet<u16>> {
        if let SetData::U16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u32(&self) -> Option<&HashSet<u32>> {
        if let SetData::U32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u64(&self) -> Option<&HashSet<u64>> {
        if let SetData::U64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u128(&self) -> Option<&HashSet<u128>> {
        if let SetData::U128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i8(&self) -> Option<&HashSet<i8>> {
        if let SetData::I8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i16(&self) -> Option<&HashSet<i16>> {
        if let SetData::I16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i32(&self) -> Option<&HashSet<i32>> {
        if let SetData::I32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i64(&self) -> Option<&HashSet<i64>> {
        if let SetData::I64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i128(&self) -> Option<&HashSet<i128>> {
        if let SetData::I128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn biguint(&self) -> Option<&HashSet<BigUint>> {
        if let SetData::BigUint(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bigint(&self) -> Option<&HashSet<BigInt>> {
        if let SetData::BigInt(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn uuid(&self) -> Option<&HashSet<Uuid>> {
        if let SetData::Uuid(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

/// Data Type for key for Map and Set
#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum DataWithoutCollection {
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
impl DataWithoutCollection {
    #[inline(always)]
    pub fn string(&self) -> Option<&String> {
        if let DataWithoutCollection::String(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bool(&self) -> Option<&bool> {
        if let DataWithoutCollection::Bool(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u8(&self) -> Option<&u8> {
        if let DataWithoutCollection::U8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u16(&self) -> Option<&u16> {
        if let DataWithoutCollection::U16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u32(&self) -> Option<&u32> {
        if let DataWithoutCollection::U32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u64(&self) -> Option<&u64> {
        if let DataWithoutCollection::U64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn u128(&self) -> Option<&u128> {
        if let DataWithoutCollection::U128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i8(&self) -> Option<&i8> {
        if let DataWithoutCollection::I8(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i16(&self) -> Option<&i16> {
        if let DataWithoutCollection::I16(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i32(&self) -> Option<&i32> {
        if let DataWithoutCollection::I32(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i64(&self) -> Option<&i64> {
        if let DataWithoutCollection::I64(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn i128(&self) -> Option<&i128> {
        if let DataWithoutCollection::I128(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn biguint(&self) -> Option<&BigUint> {
        if let DataWithoutCollection::BigUint(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn bigint(&self) -> Option<&BigInt> {
        if let DataWithoutCollection::BigInt(data) = self {
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn uuid(&self) -> Option<&Uuid> {
        if let DataWithoutCollection::Uuid(data) = self {
            Some(data)
        } else {
            None
        }
    }
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
///
/// ### TODO
/// 추후 개발시, 모든 operation에 대해 필요한 datacell을 건내준 다음
/// 처음 operate를 수행하며 modified number를 기록하고
/// 이후 타이밍이 되었을때 modified number를 비교하여
/// 수정되지 않았을경우 그대로 적용하고
/// 수정되었을경우 다시 operation을 수행하도록 해야합니다.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataCell(Data, u64);

impl DataCell {
    pub fn new(data: Data) -> Self {
        Self(data, fastrand::u64(..))
    }
    pub fn get(&self) -> &Data {
        &self.0
    }
    pub fn get_modified_number(&self) -> u64 {
        self.1
    }
    pub fn set(&mut self, data: Data) {
        self.0 = data;
        self.1 = fastrand::u64(..);
    }
}
