use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    x: i32,
    y: i32,
    width: u32,
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
            width,
            max_x: game_width - width,
        }
    }


    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(0, self)
    }

    pub fn move_horizontally(&mut self, offset: i32) {
        self.x += offset;
        if self.x < 0 { self.x = 0 };
        if self.x >= self.max_x as i32 { self.x = self.max_x as i32 }
    }

    pub fn bullet_spawn_position(&self, bullet_width: i32, bullet_height: i32) -> (i32, i32) {
        return (self.x + self.width as i32 / 2 - bullet_width / 2, self.y - bullet_height);
    }
}

impl Positioned for PlayerShip {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}