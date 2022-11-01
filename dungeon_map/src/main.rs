#![warn(clippy::pedantic)]

// START: prelude
mod map;
mod player;
mod map_builder ;
mod camera ;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;

}
// END: prelude

use prelude::*;


// START: player_state
struct State {
    map: Map,
    player: Player,
    camera:  Camera,
}

impl State {
    fn new() -> Self {
        Self {
            map : Map::new(),
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
// END: player_state

fn main() -> BError {
    let context = BTermBuilder::new()// <callout id="co.dungeongfx.newterm" />
    .with_title("Dungeon Crawler")
    .with_fps_cap(30.0)
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // <callout id="co.dungeongfx.dimensions" />
    .with_tile_dimensions(32, 32) // <callout id="co.dungeongfx.tiledimensions" />
    .with_resource_path("resources/") // <callout id="co.dungeongfx.resources" />
    .with_font("dungeonfont.png", 32, 32) // <callout id="co.dungeongfx.font" />
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // <callout id="co.dungeongfx.con1" />
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
        "dungeonfont.png") // <callout id="co.dungeongfx.con2" />
    .build()?;
//END: layers

    main_loop(context, State::new())
}
