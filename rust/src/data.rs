use godot::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

static NEXT_UID: AtomicI32 = AtomicI32::new(1);

#[derive(GodotConvert, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[godot(transparent)]
pub struct GridIdx(pub i64);
impl GridIdx {
    pub fn dist(a: GridIdx, b: GridIdx) -> i64 {
        (a.0 - b.0).abs()
    }
}

#[derive(GodotConvert, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[godot(transparent)]
pub struct UID(u32);
impl UID {
    pub fn new() -> Self {
        let uid = NEXT_UID.fetch_add(1, Ordering::Relaxed);
        UID(uid as u32)
    }

    pub fn from(id: u32) -> Self {
        UID(id)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn null() -> Self {
        UID(0)
    }
}
