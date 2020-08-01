use tetra::graphics::{self, Color, Texture};
use tetra::{Context, ContextBuilder, State};

struct GameState {
    background_texture: Texture,
}


impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        Ok(())
    }
}
fn main() -> tetra::Result {
    ContextBuilder::new("JuicyGame", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(|_ctx| {
            let backgroubd = Texture::new(_ctx, "./data/global_sprotesheet.png")?;
            Ok(GameState {background_texture}))
        }
}
