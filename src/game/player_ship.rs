use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    x: i32,
    y: i32,
    max_x: u32,
}

impl PlayerShip {
    pub fn new(dimensions: (u32, u32), game_dimensions: (u32, u32)) -> Self {
        let (width, height) = dimensions;
        let (game_width, game_height) = game_dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        Self {
            x: ship_x as i32,
            y: ship_y as i32,
            max_x: game_width - width,
        }
    }


    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite(0, self.x as i32, self.y as i32)
    }

    pub fn move_horizontally(&mut self, offset: i32) {
        self.x += offset;
        if self.x < 0 { self.x = 0 };
        if self.x >= self.max_x as i32 { self.x = self.max_x as i32 }
    }
}