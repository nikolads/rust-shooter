use ggez::{Context, GameResult};
use ggez::graphics::{self, Vector2, Point2};
use ggez::nalgebra as na;

use assets::Assets;

#[derive(Debug)]
pub enum PlayerState {
    Normal,
    Shooting,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub pos: Point2,
    pub time_until_next_shot: f32,
    velocity: Vector2,
}

impl Player {
    pub const SHOT_TIMEOUT: f32 = 1.0;
    pub const SPEED: f32 = 500.0;

    pub fn new(pos: Point2) -> Self {
        Player {
            state: PlayerState::Normal,
            pos,
            velocity: Vector2::new(0.0, 0.0),
            time_until_next_shot: Self::SHOT_TIMEOUT,
        }
    }

    pub fn update(&mut self, amount: f32, seconds: f32, max_right: f32) {
        let new_pos = self.pos.x + Self::SPEED * seconds * amount;
        self.pos.x = na::clamp(new_pos, 0.0, max_right);
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        match self.state {
            PlayerState::Normal => {
                graphics::draw_ex(ctx, &assets.ferris_normal_image, graphics::DrawParam {
                    dest: self.pos,
                    scale: Point2::new(0.95, 0.95),
                    offset: Point2::new(0.5, 1.0),
                    .. Default::default()
                })?;
            },

            PlayerState::Shooting => {
                graphics::draw_ex(ctx, &assets.ferris_shooting_image, graphics::DrawParam {
                    dest: self.pos,
                    offset: Point2::new(0.545, 0.96),
                    .. Default::default()
                })?;
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Shot {
    pub pos: Point2,
    pub is_alive: bool,
    velocity: Vector2,
}

impl Shot {
    pub fn new(pos: Point2) -> Self {
        Shot {
            pos,
            is_alive: true,
            velocity: Vector2::new(0.0, -500.0),
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos += self.velocity * seconds;
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw_ex(ctx, &assets.shot_image, graphics::DrawParam {
            dest: self.pos,
            .. Default::default()
        })
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub pos: Point2,
    pub is_alive: bool,
    label: String,
    velocity: Vector2,
    sprite: Box<dyn Sprite>,
}

pub trait Sprite: std::fmt::Debug {
    fn draw(&mut self, center: Point2, ctx: &mut Context) -> GameResult<()>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str, ctx: &mut Context) -> GameResult<TextSprite> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
        let text = graphics::Text::new(ctx, label, &font)?;
        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, center: Point2, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(ctx, &self.text, graphics::DrawParam {
            dest: center,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        })
    }

    fn width(&self) -> u32 { self.text.width() }
    fn height(&self) -> u32 { self.text.height() }
}

impl Enemy {
    pub fn new(label: &str, pos: Point2, speed: f32, sprite: Box<dyn Sprite>) -> GameResult<Self> {
        let label = String::from(label);

        Ok(Enemy {
            pos, label, sprite,
            is_alive: true,
            velocity: Vector2::new(0.0, speed),
        })
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos += self.velocity * seconds;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.sprite.draw(self.pos, ctx)
    }

    pub fn bounding_rect(&self) -> graphics::Rect {
        let left   = self.pos.x - self.sprite.width()  as f32 / 2.0;
        let right  = self.pos.x + self.sprite.width()  as f32 / 2.0;
        let top    = self.pos.y - self.sprite.height() as f32 / 2.0;
        let bottom = self.pos.y + self.sprite.height() as f32 / 2.0;

        graphics::Rect::new(left, top, right - left, bottom - top)
    }
}
