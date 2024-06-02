use std::collections::VecDeque;

use bracket_lib::prelude::*;
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Wolfbang").build()?;
    main_loop(context, State::new())
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0;
enum GameMode {
    Menu,
    Playing,
    End,
}
struct State {
    player: Student,
    mode: GameMode,
    frametime: f32,
    obstacle: VecDeque<Obstacle>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Student::new(25),
            frametime: 0.0,
            mode: GameMode::Menu,
            obstacle: {
                let mut z = VecDeque::new();
                z.push_front(Obstacle::new(65, 0));
                z
            },
        }
    }

    fn render_add_remove_obstacles(&mut self, ctx: &mut BTerm) {
        match self.obstacle.back() {
            Some(x) => {
                if x.x <= 55 {
                    self.obstacle
                        .push_back(Obstacle::new(SCREEN_WIDTH - 1, self.player.score))
                }
            }
            None => {}
        }
        match self.obstacle.front() {
            Some(x) => {
                if x.x == 0 {
                    if !self
                        .player
                        .hitbox((x.gap_y + x.size / 2, x.gap_y - x.size / 2))
                    {
                        self.mode = GameMode::End
                    };
                    match self.obstacle.pop_front() {
                        _ => self.player.score += 1,
                    }
                }
            }
            None => {}
        }

        for i in self.obstacle.iter_mut() {
            i.render(ctx);
            i.x -= 1
        }
    }

    fn restart(&mut self) {
        self.player = Student::new(25);
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
        self.frametime += ctx.frame_time_ms;
        if self.frametime > FRAME_DURATION {
            self.frametime = 0.0;
            self.player.gravity_and_move();
        }
        self.render_add_remove_obstacles(ctx);
        self.player.render(ctx);
        self.player.show_score(ctx);
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }
    }
}

struct Student {
    y: i32,
    velocity: f32,
    score: i32,
}

impl Student {
    fn new(y: i32) -> Self {
        Student {
            y,
            velocity: 1.8,
            score: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, (28, 49, 68), (126, 161, 107), to_cp437('►'))
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
        if self.y < 0 {
            self.y = 0
        } else if self.y > 49 {
            self.y = 49;
        }
    }

    fn flap(&mut self) {
        self.velocity = -6.0;
    }

    fn hitbox(&mut self, gap: (i32, i32)) -> bool {
        return self.y > gap.1 && self.y < gap.0;
    }

    fn show_score(&self, ctx: &mut BTerm) {
        ctx.print(SCREEN_WIDTH - 9, 0, format!("score:{}", self.score))
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
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
            ctx.set(self.x, i, (112, 22, 30), (126, 161, 107), to_cp437('▓'))
        }

        for i in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(self.x, i, (112, 22, 30), (126, 161, 107), to_cp437('▓'));
        }
    }
}
