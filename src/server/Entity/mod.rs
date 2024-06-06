use bevy::prelude::Component;
pub mod MovingObject;
#[derive(Component)]
pub(crate) struct Entity{
    pub(crate) world_position: [f32; 2],
    pub(crate) world_angle: f32,
    pub(crate) size : f32,
    pub(crate) lock : bool,
    pub(crate) name : String,
    pub(crate) color : bevy::render::color::Color,
}

impl Entity{

    pub fn new(&mut self, world_position: [f32; 2], world_angle:f32, size:f32){
        self.size = size;
        self.world_angle = world_angle;
        self.world_position = world_position;
    }

    pub fn set_world_position(&mut self, x : f32, y : f32){
        self.world_position[0] = x;
        self.world_position[1] = y
    }
    pub fn get_world_position(&self) -> [f32; 2] {
       return self.world_position
    }

    pub fn set_angle(&mut self, angle : f32){
        self.world_angle = angle
    }

    pub fn get_angle(&self) -> &f32{
        return &self.world_angle
    }

    pub fn set_size(&mut self, size : f32){
        self.size = size
    }

    pub fn get_name(&self) -> &String {
        return &self.name
    }

    pub fn set_color(&mut self, color : bevy::render::color::Color){
        self.color = color
    }

    fn get_color(&self) -> &bevy::render::color::Color {
        return &self.color
    }

}