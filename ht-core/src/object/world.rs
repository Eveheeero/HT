use crate::{Data, DataCell, DataCode, Object};
use bevy::ecs::system::Resource;
use intmap::IntMap;

/// 게임 세상 전부
#[derive(Clone, Default, Debug, Resource)]
pub struct HtWorld(IntMap<Object>);

impl HtWorld {
    /// 객체를 추가합니다.
    ///
    /// 아이디가 0이거나 None일경우, 임시객체로 간주하고 새로운 아이디를 부여합니다.
    ///
    /// ### Returns
    /// - `Option<Object>`: 이미 존재하는 객체가 있을 경우, 그 객체를 반환합니다.
    /// - `u64`: 새로 추가된 객체의 아이디를 반환합니다.
    pub fn insert(&mut self, mut value: Object) -> (Option<Object>, u64) {
        let id = value.get(DataCode::Id);
        // 아이디가 설정되지 않은경우
        if id.is_none() || id.unwrap().get().u64().unwrap() == &0 {
            loop {
                let id = fastrand::u64(1..);
                if self.0.get(id).is_some() {
                    continue;
                }
                value.insert(DataCode::Id, DataCell::new(Data::U64(id)));
                return (self.0.insert(id, value), id);
            }
        } else {
            let id = *id.unwrap().get().u64().unwrap();
            return (self.0.insert(id, value), id);
        }
    }
    pub fn get(&self, key: u64) -> Option<&Object> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: u64) -> Option<&mut Object> {
        self.0.get_mut(key)
    }
    pub fn remove(&mut self, key: u64) -> Option<Object> {
        self.0.remove(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &Object)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u64, &mut Object)> {
        self.0.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
