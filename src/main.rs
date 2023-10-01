#![feature(extract_if)]

use macroquad::prelude::*;

const WIDTH: i32 = 948;
const HEIGHT: i32 = 533;
const HALF_WIDTH: f32 = 948.0 / 2.0;
const HALF_HEIGHT: f32 = 533.0 / 2.0;

pub struct DrawState<'a> {
	pub font: Option<&'a Font>,
}

pub enum Tick {
	Modal,
	Passthrough,
	Done,
	Replace(Box<dyn State>),
}

pub trait State {
	fn draw(&mut self, state: &DrawState);
	fn tick(&mut self) -> Tick;
}

fn window_conf() -> Conf {
    Conf {
        window_title: "LD54".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

trait WithAlpha {
	fn with_alpha(self, alpha: f32) -> Self;
}

impl WithAlpha for Color {
	fn with_alpha(self, alpha: f32) -> Self {
		Self {
			r: self.r,
			g: self.g,
			b: self.g,
			a: alpha,
		}
	}
}

struct Item {
}

#[derive(Default)]
struct Player {
	inventory: [[Option<Item>; 5]; 5],
}

#[derive(Default)]
struct GridCell {
	item: Option<Item>,
	entity: Option<u32>,
}

#[derive(Default)]
struct Level {
	pub grid: [[GridCell; 50]; 50],
}

#[derive(Default)]
struct Game {
	pub entities: HashMap<u32, Entity>,
	pub player: Player,
	pub level: Level,
}

impl State for Game {
	fn draw(&mut self, &DrawState { font }: &DrawState) {
	}

    fn tick(&mut self) -> Tick {
        let delta = get_frame_time();

		Tick::Modal
	}
}

struct DialogueBox {
    pub speaker: &'static str,
    pub text: &'static str,
}

struct Dialogue {
    pub boxes: Vec<DialogueBox>,
    pub fade: f32,
}

impl Dialogue {
    pub fn new(mut boxes: Vec<DialogueBox>) -> Self {
		boxes.reverse();
        Self { boxes, fade: 0.0 }
    }
}

impl State for Dialogue {
    fn draw(&mut self, &DrawState { font }: &DrawState) {
        let Some(this) = self.boxes.iter().rev().next() else {
			return;
		};

        let popup = 40.0 * (1.0 - (self.fade * 3.0).clamp(0.0, 1.0)).powf(3.0);
        let alpha = 1.0 - (1.0 - (self.fade * 3.0).clamp(0.0, 1.0)).powf(3.0);

		let M = 5.0;
		draw_rectangle(
			-HALF_WIDTH + 20.0 - M,
			HALF_HEIGHT - 145.0 - M + popup,
			(WIDTH as f32) + (-20.0 + M) * 2.0,
			115.0 + M,
			BLACK.with_alpha(alpha * 0.3),
		);

        draw_text_ex(
            &this.speaker,
            -HALF_WIDTH + 30.0,
            HALF_HEIGHT - 125.0 + popup,
            TextParams {
                font_size: 40,
                font,
				color: WHITE.with_alpha(alpha),
                ..Default::default()
            },
        );

        for (i, line) in this.text.split('\n').enumerate() {
            draw_text_ex(
                line,
                -HALF_WIDTH + 50.0,
                HALF_HEIGHT - 80.0 + (i as f32) * 30.0 + popup,
                TextParams {
                    font_size: 30,
                    font,
					color: WHITE.with_alpha(alpha),
                    ..Default::default()
                },
            );
        }
    }

    fn tick(&mut self) -> Tick {
        self.fade += get_frame_time();

		if self.fade > 0.3 && (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Right) || is_mouse_button_pressed(MouseButton::Left)) {
			self.fade = 0.0;
			self.boxes.pop();
		}

		if self.boxes.is_empty() {
			return Tick::Done;
		}

		Tick::Modal
	}
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("data/font.ttf").await.ok();
    let font = font.as_ref();

	let PLAYERNAME = "Nat";
	let ANTAGNAME = "Evillé";

	let mut state: Vec<Box<dyn State>> = vec![];

	state.push(Box::new(Game::default()));

	state.push(Box::new(Dialogue::new(vec![
        DialogueBox {
            speaker: ANTAGNAME,
            text: "What's our plan?",
        },
        DialogueBox {
            speaker: ANTAGNAME,
            text: "I hate you",
        },
    ])));

    loop {
        let camera = Camera2D {
            zoom: Vec2::new(2.0 / screen_width(), 2.0 / screen_height()),
            offset: Vec2::new(0.0, 0.0),
            ..Default::default()
        };
        set_camera(&camera);

        let gl = unsafe { get_internal_gl().quad_gl };
        clear_background(GRAY);

		let draw_state = DrawState { font };

		let mut from_last = 0;
		while let Some(here) = state.iter_mut().rev().skip(from_last).next() {
			match here.tick() {
				Tick::Modal => break,
				Tick::Passthrough => {},
				Tick::Done => {
					state.remove(state.len() - from_last - 1);
					continue;
				},
				Tick::Replace(new) => {
					*here = new
				},
			}

			from_last += 1;
		}

		for item in state.iter_mut().rev() {
			item.draw(&draw_state);
		}

        next_frame().await
    }
}
