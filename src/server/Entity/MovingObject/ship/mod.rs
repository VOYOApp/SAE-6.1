use bevy::prelude::Component;
use crate::server::Entity::MovingObject::MovingObject;

#[derive(Component)]
pub(crate) struct ship{
    pub(crate) body: MovingObject,
    pub(crate) right_wheel: f32,
    pub(crate) left_wheel: f32,
    pub(crate) gun_orientation: [f32; 2],
}

impl ship{
    pub fn new(&mut self,world_position: [f32; 2], world_angle: f32,size: f32){
        self.body.new(world_position, world_angle,size)
    }

    pub fn set_right_wheel(&mut self, right_wheel: f32){
        self.right_wheel = right_wheel
    }

    pub fn set_left_wheel(&mut self, left_wheel: f32){
        self.left_wheel = left_wheel
    }

    pub fn get_right_wheel(&self) -> &f32{
        return &self.right_wheel
    }

    pub fn get_left_wheel(&self) -> &f32{
        return &self.left_wheel
    }

    pub fn get_body(&self) -> &MovingObject{
        return &self.body
    }

    pub fn set_body(&mut self, body: MovingObject){
        self.body = body
    }

    pub fn set_gun_orientation(&mut self, gun_orientation: [f32; 2]){
        self.gun_orientation = gun_orientation
    }

    pub fn get_gun_orientation(&self) -> &[f32; 2]{
        return &self.gun_orientation
    }



}