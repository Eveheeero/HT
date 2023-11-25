use crate::Data;

use super::DataCell;
use intmap::IntMap;
use serde::{Deserialize, Serialize};

/// 실제 게임 데이터 객체에 대한 데이터들
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Object(IntMap<DataCell>);

#[repr(u64)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum DataCode {
    /// 객체 아이디, u64, 0이면 임시객체
    Id,
    /// 객체 이름, String
    Name,
    /// 객체 체력, None or BigUInt
    Hp,
    /// 객체 설명, None or String
    Description,
    /// 종족명, 무기물이면 None, 그 외는 String
    RaceName,
    /// 종족 코드, 무기물이면 None, 그 외는 u16
    RaceCode,
    /// 객체 위치, f64 -> (f32, f32)
    Position,
    /// 객체 이미지, None or u64(bevy 핸들 id)
    Sprite,
    /// 객체 무게, f32
    Weight,
    /// 객체가 가지고 있는 물체들, None or Set<u64>
    Inventory,
}

impl Object {
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

impl Object {
    pub fn get_position(&self) -> Option<(f32, f32)> {
        self.get(DataCode::Position).map(DataCell::get).map(|x| {
            if let Data::F64(data) = x {
                Self::to_position(*data)
            } else {
                unreachable!("좌표데이터는 f64여야합니다.")
            }
        })
    }
    pub fn set_position(&mut self, position: (f32, f32)) {
        let position = Data::F64(Self::from_position(position));
        if let Some(cell) = self.get_mut(DataCode::Position) {
            cell.set(position);
        } else {
            self.insert(DataCode::Position, DataCell::new(position));
        }
    }

    #[inline(always)]
    fn to_position(position: f64) -> (f32, f32) {
        unsafe { std::mem::transmute::<f64, (f32, f32)>(position) }
    }
    #[inline(always)]
    fn from_position(position: (f32, f32)) -> f64 {
        unsafe { std::mem::transmute::<(f32, f32), f64>(position) }
    }
}
