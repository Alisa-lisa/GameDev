use rayon::prelude::*;
use shipyard::{
    AllStoragesViewMut, EntitiesViewMut, IntoIter, NonSendSync, Shiperator, View, ViewMut, World,
};
use std::time::Duration;
use tetra::graphics::animation::Animation;
use tetra::graphics::{self, Color, DrawParams, Drawable, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const HEIGHT: f32 = 1280.0;
const WIDTH: f32 = 720.0;

#[derive(Debug)]
struct Position(Vec2<f32>);

#[derive(Debug)]
struct Velocity(Vec2<f32>);

#[derive(Debug)]
struct Player;

#[derive(Debug, PartialEq)]
enum PlayerState {
    Idle,
    Running,
}

#[derive(Debug)]
struct PlayerAnimation {
    state: PlayerState,
    running_animation: Animation,
    breath_cycle: f32,
    breath_cycle_length: f32,
}

struct GameState {
    world: World,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // Draw Textures
        self.world.run(
            |positions: View<Position>, textures: NonSendSync<View<Texture>>| {
                (&positions, &*textures).iter().for_each(|(pos, tex)| {
                    graphics::draw(
                        ctx,
                        tex,
                        DrawParams::new()
                            .position(pos.0)
                            .clip(Rectangle::new(0., 176., 160., 128.))
                            .scale((HEIGHT / 160.0, WIDTH / 128.0).into()),
                    );
                });
            },
        );

        // Draw Animations
        self.world.run(
            |positions: View<Position>,
             velocities: View<Velocity>,
             mut player_animations: NonSendSync<ViewMut<PlayerAnimation>>| {
                (&positions, &velocities, &mut *player_animations)
                    .iter()
                    .for_each(|(pos, velocities, player_animation)| {
                        player_animation.running_animation.advance(ctx);
                        let origin = (
                            player_animation.running_animation.frames()[0].width / 2.,
                            player_animation.running_animation.frames()[0].height,
                        )
                            .into();
                        let breath = 1.0 + ezing::expo_inout((player_animation.breath_cycle * 2.0 - 1.0).abs()) * 0.2;
                        let scale = (3.0 * velocities.0.x.signum(), 3.0 * breath).into();
                        match player_animation.state {
                            PlayerState::Idle => {
                                graphics::draw(
                                    ctx,
                                    &player_animation.running_animation,
                                    DrawParams::new()
                                        .origin(origin)
                                        .position(pos.0)
                                        .scale(scale),
                                );
                            }
                            PlayerState::Running => {
                                graphics::draw(
                                    ctx,
                                    &player_animation.running_animation,
                                    DrawParams::new()
                                        .origin(origin)
                                        .position(pos.0)
                                        .scale(scale),
                                );
                            }
                        }
                    });
            },
        );
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.world.run(
            |mut positions: ViewMut<Position>,
             mut velocities: ViewMut<Velocity>,
             mut player_animations: NonSendSync<ViewMut<PlayerAnimation>>,
             players: View<Player>| {
                (
                    &mut positions,
                    &mut velocities,
                    &mut *player_animations,
                    &players,
                )
                    .iter()
                    .for_each(|(pos, mut velocity, mut player_animation, _)| {
                        player_animation.breath_cycle =
                            (player_animation.breath_cycle + 0.01) % 1.0;
                        if input::is_key_down(&ctx, Key::Left) || input::is_key_down(&ctx, Key::A) {
                            velocity.0.x = (velocity.0.x - 0.5).max(-5.0);
                        } else if input::is_key_down(&ctx, Key::Right)
                            || input::is_key_down(&ctx, Key::D)
                        {
                            velocity.0.x = (velocity.0.x + 0.5).min(5.0);
                        } else {
                            velocity.0.x -= velocity.0.x.abs().min(0.5) * velocity.0.x.signum();
                        }

                        pos.0 += velocity.0;

                        if velocity.0.x.abs() > 0.0 {
                            player_animation.state = PlayerState::Running;
                        } else {
                            player_animation.running_animation.restart();
                            player_animation.state = PlayerState::Idle;
                        }
                    });
            },
        );
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let spritesheet_tex = Texture::new(ctx, "./data/global_spritesheet.png")?;
        let player_animation_run = Animation::new(
            spritesheet_tex.clone(),
            Rectangle::row(0.0, 13.0, 32.0, 19.0)
                .take(3)
                .chain(Rectangle::row(0.0, 44.0, 32.0, 19.0).take(2))
                .collect(),
            Duration::from_secs_f64(0.1),
        );
        // Rocket: 66, 68, 12, 8
        let world = World::new();
        world.run(
            |mut entities: EntitiesViewMut,
             mut positions: ViewMut<Position>,
             mut velocities: ViewMut<Velocity>,
             mut player_animations: NonSendSync<ViewMut<PlayerAnimation>>,
             mut players: ViewMut<Player>,
             mut textures: NonSendSync<ViewMut<Texture>>| {
                let _player = entities.add_entity(
                    (
                        &mut positions,
                        &mut velocities,
                        &mut *player_animations,
                        &mut players,
                    ),
                    (
                        Position((100., 600.).into()),
                        Velocity((0., 0.).into()),
                        PlayerAnimation {
                            state: PlayerState::Idle,
                            running_animation: player_animation_run,
                            breath_cycle: 0.0,
                            breath_cycle_length: 2.0,
                        },
                        Player {},
                    ),
                );
                let _background = entities.add_entity(
                    (&mut positions, &mut *textures),
                    (Position((0., 0.).into()), spritesheet_tex),
                );
            },
        );
        Ok(GameState { world })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("JuicyGame", HEIGHT as i32, WIDTH as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
