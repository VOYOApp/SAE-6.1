use rapier2d::prelude::*;

/// Represents an obstacle in the game.
///
/// An obstacle has a position and a handle to its collider.
pub struct Obstacle {
    pub position: (f64, f64),
    pub collider_handle: ColliderHandle,
}

impl Obstacle {
    /// Creates a new obstacle.
    ///
    /// # Parameters
    /// - `position`: A tuple representing the (x, y) position of the obstacle.
    /// - `collider_handle`: The handle to the collider associated with this obstacle.
    ///
    /// # Returns
    /// A new instance of `Obstacle`.
    pub fn new(position: (f64, f64), collider_handle: ColliderHandle) -> Self {
        Self {
            position,
            collider_handle,
        }
    }
}
