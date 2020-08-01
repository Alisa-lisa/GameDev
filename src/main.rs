use tetra::graphics::{self, Color, Texture};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;


struct GameState {
    background_texture: Texture,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.background_texture, Vec2::new(0.0, 0.0));
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let background_texture = Texture::new(ctx, "./data/global_spritesheet.png")?;
        Ok(GameState { background_texture })
    }

}

fn main() -> tetra::Result {
    ContextBuilder::new("JuicyGame", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
