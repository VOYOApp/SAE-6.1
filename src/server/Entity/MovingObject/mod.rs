use crate::Entity;
pub mod ship;
pub(crate) struct MovingObject {
   pub(crate) entity : Entity
}

impl MovingObject{

    fn new(&mut self,world_position: [f32; 2], world_angle: f32,size: f32){
        self.entity.new(world_position, world_angle,size)
    }

}