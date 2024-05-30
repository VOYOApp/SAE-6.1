struct Entity{
    world_position: Vec<>,
    world_angle: f32,
    size : f32,
    lock : bool,
    name : String,
    color : String //  TODO : Remplacer par un meilleur type pour que cela fonctionne
}

impl Entity{

    fn new(&mut self, world_position: Vec<>, world_angle:f32,size:f32){
        &self.size = size;
        &self.world_angle = world_angle;
        &self.world_position = world_position;
    }

    fn set_world_position(&mut self, x : f32, y : f32){
        &self.world_position.x = x;
        &self.world_position.y = y
    }
    fn get_world_position(&self) -> &Vec<> {
       &self.world_position
    }

    fn set_angle(&mut self, angle : f32){
        &self.world_angle = angle
    }

    fn get_angle(&self) -> &f32{
        &self.world_angle
    }

    fn set_size(&mut self, size : f32){
        &self.size = size
    }

    fn get_name(&self) -> String {
        &self.name
    }

    fn set_color(&mut self, color : String){
        &self.color = color
    }

    fn get_color(&self) -> String {
        &self.color
    }

}