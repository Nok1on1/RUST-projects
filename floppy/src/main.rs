use std::{collections::VecDeque, process::exit};

use bracket_lib::prelude::*;
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Wolfbang")
        .with_fps_cap(60.0)
        .build()?;
    main_loop(context, State::new())
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0;
enum GameMode {
    Menu,
    Playing,
    End,
    Pause,
    Debug,
}
struct Settings {
    frametime: f32,
}

impl Settings {
    fn show_fps(&self, ctx: &mut BTerm) {
        ctx.print(3, 0, format!("framerate:{}", ctx.fps))
    }
    fn show_velocity(&self, state: &State, ctx: &mut BTerm) {
        ctx.print(1, 1, state.player.velocity);
    }

    fn border(&self, ctx: &mut BTerm) {
        for i in 1..SCREEN_WIDTH - 1 {
            ctx.set(i, 0, (112, 22, 30), (126, 161, 107), to_cp437('═'));
            ctx.set(
                i,
                SCREEN_HEIGHT - 1,
                (112, 22, 30),
                (126, 161, 107),
                to_cp437('═'),
            )
        }
        for i in 1..SCREEN_HEIGHT - 1 {
            ctx.set(0, i, (112, 22, 30), (126, 161, 107), to_cp437('║'));
            ctx.set(
                SCREEN_WIDTH - 1,
                i,
                (112, 22, 30),
                (126, 161, 107),
                to_cp437('║'),
            )
        }

        ctx.set(
            SCREEN_WIDTH - 1,
            SCREEN_HEIGHT - 1,
            (112, 22, 30),
            (126, 161, 107),
            to_cp437('╝'),
        );
        ctx.set(
            0,
            SCREEN_HEIGHT - 1,
            (112, 22, 30),
            (126, 161, 107),
            to_cp437('╚'),
        );
        ctx.set(
            SCREEN_WIDTH - 1,
            0,
            (112, 22, 30),
            (126, 161, 107),
            to_cp437('╗'),
        );
        ctx.set(0, 0, (112, 22, 30), (126, 161, 107), to_cp437('╔'));
    }
}
struct State {
    player: Player,
    mode: GameMode,
    frametime: f32,
    obstacle: VecDeque<Obstacle>,
    setting: Settings,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Debug => self.debug(ctx),
            GameMode::Pause => self.pause(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(25),
            frametime: 0.0,
            mode: GameMode::Menu,
            obstacle: {
                let mut z = VecDeque::new();
                z.push_front(Obstacle::new(65.0, 0));
                z
            },
            setting: Settings { frametime: 0.0 },
        }
    }

    fn render_add_remove_obstacles(&mut self, ctx: &mut BTerm) {
        match self.obstacle.back() {
            Some(x) => {
                if x.x <= 55.0 {
                    self.obstacle
                        .push_back(Obstacle::new((SCREEN_WIDTH - 1) as f32, self.player.score))
                }
            }
            None => {}
        }
        match self.obstacle.front() {
            Some(x) => {
                if x.x == 0.0 {
                    match self.obstacle.pop_front() {
                        _ => self.player.score += 1,
                    }
                }
            }
            None => {}
        }

        for i in self.obstacle.iter_mut() {
            i.render(ctx);
            i.x -= 0.5
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(25);
        self.frametime = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg((89, 111, 98));
        ctx.print_centered(15, "Welcome to floppy");
        ctx.print_centered(20, "(P) Play Game");
        ctx.print_centered(21, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                VirtualKeyCode::D => self.mode = GameMode::Debug,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg((89, 111, 98));
        ctx.print_centered(15, "You Died!");
        ctx.print_centered(20, "(P) Play Game");
        ctx.print_centered(21, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg((126, 161, 107));
        self.player.hitbox(
            &mut self.mode,
            match self.obstacle.front() {
                Some(x) => x,
                None => exit(001),
            },
        );
        self.frametime += ctx.frame_time_ms;
        if self.frametime > FRAME_DURATION {
            self.frametime = 0.0;
            self.player.gravity_and_move();
        }
        self.render_add_remove_obstacles(ctx);
        self.player.render(ctx);
        self.player.show_score(ctx);
        self.setting.border(ctx);
        self.player.show_score(ctx);
        self.setting.show_fps(ctx);
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::P => self.mode = GameMode::Pause,
                _ => {}
            }
        }
    }

    fn debug(&mut self, ctx: &mut BTerm) {
        self.setting.frametime += ctx.frame_time_ms;
        if self.setting.frametime > FRAME_DURATION {
            self.player.gravity_and_move();
            self.setting.frametime = 0.0;
            ctx.cls_bg((126, 161, 107));
            ctx.print(25, 25, ctx.frame_time_ms);
            self.render_add_remove_obstacles(ctx);
            self.setting.border(ctx);
            self.setting.show_velocity(self, ctx);
            self.player.render(ctx);
            self.player.show_score(ctx);
            self.setting.show_fps(ctx);
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::P => self.mode = GameMode::Pause,
                _ => {}
            }
        }
    }
    fn pause(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Playing,
                _ => {}
            }
        }
    }
}

struct Player {
    y: i32,
    velocity: f32,
    score: i32,
}

impl Player {
    fn new(y: i32) -> Self {
        Player {
            y,
            velocity: 1.8,
            score: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(1, self.y, (28, 49, 68), (126, 161, 107), to_cp437('►'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 8.0 {
            self.velocity += 1.0;
        }
        let sign = self.velocity.is_sign_positive();
        self.y += if sign {
            f32::sqrt(self.velocity) as i32
        } else {
            -f32::sqrt(i32::abs(self.velocity as i32) as f32) as i32
        };
        if self.y < 1 {
            self.y = 1
        } else if self.y > 48 {
            self.y = 48;
        }
    }

    fn flap(&mut self) {
        self.velocity = -6.0;
    }

    fn hitbox(&mut self, gamemode: &mut GameMode, obstacle: &Obstacle) {
        if obstacle.x == 0.0 {
            if !(self.y > (obstacle.gap_y - obstacle.size / 2)
                && self.y < (obstacle.gap_y + obstacle.size / 2))
            {
                *gamemode = GameMode::End;
            };
        }
    }

    fn show_score(&self, ctx: &mut BTerm) {
        ctx.print(SCREEN_WIDTH - 9, 0, format!("score:{}", self.score))
    }
}

struct Obstacle {
    x: f32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: f32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(20, 40),
            size: i32::max(2, 16 - score / 15),
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        let half_size = self.size / 2;
        for i in 0..self.gap_y - half_size {
            ctx.set(
                self.x as i32,
                i,
                (112, 22, 30),
                (126, 161, 107),
                to_cp437('▓'),
            )
        }

        for i in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                self.x as i32,
                i,
                (112, 22, 30),
                (126, 161, 107),
                to_cp437('▓'),
            );
        }
    }
}
