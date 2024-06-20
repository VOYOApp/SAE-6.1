use rapier2d::prelude::*;

pub struct Obstacle {
    pub position: (f64, f64),
    pub collider_handle: ColliderHandle,
}

impl Obstacle {
    pub fn new(position: (f64, f64), collider_handle: ColliderHandle) -> Self {
        Self {
            position,
            collider_handle,
        }
    }
}
