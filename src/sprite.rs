
trait GM_Sprite_T: GM_Update + GM_Draw {
}



struct GM_Sprite {
    sprite_sheet_id: usize,
    position: GM_Position,
    velocity: GM_Velocity,
    collision_id: usize,
    animation_id: usize,
    path_id; usize,
}

impl GM_Sprite {
    pub fn new() -> GM_Sprite {
        GM_Sprite {
            sprite_sheet_id: 0,
            position: GM_Position::new(),
            velocity: GM_Velocity::new(),
            acceleration: GM_Acceleration::new(),
            collision_id: 0,
            animation_id: 0,
            path_id: 0,
        }
    }
}

impl GM_Update_T for GM_Sprite {
    fn update(&mut self, time_elapsed: u16) {
        self.velocity.update(self.acceleration, time_elapsed);
        self.position.update(self.velocity, time_elapsed);
    }    
}
