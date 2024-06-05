use crate::Entity;
struct MovingObject {
   entity : Entity
}

impl MovingObject{

    fn new(&mut self,world_position: Vec<>, world_angle: f32,size: f32){
        &self.entity.new(world_position, world_angle,size);
        //  TODO : Add physics properties
    }

}