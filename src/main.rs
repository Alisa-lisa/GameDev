use tetra::graphics::{self, Color, Texture, DrawParams, Rectangle};
use tetra::graphics::animation::Animation;
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use std::time::Duration;

struct GameState {
    background_texture: Texture,
    player: Animation,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.background_texture, DrawParams::new()
                       .clip(Rectangle::new(0., 176., 160., 128.))
                       .scale((5.,5.).into()));
        graphics::draw(ctx, &self.player, DrawParams::new().scale((3.0, 3.0).into()));
        self.player.advance(ctx); 
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let background_texture = Texture::new(ctx, "./data/global_spritesheet.png")?;
        let player = Animation::new(background_texture.clone(), Rectangle::row(0.0, 13.0, 32.0, 19.0).take(3).collect(), Duration::from_secs_f64(0.1));
        Ok(GameState { background_texture, player })
    }

}

fn main() -> tetra::Result {
    ContextBuilder::new("JuicyGame", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
