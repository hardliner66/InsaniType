use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;
use rand::prelude::*;

const DEFAULT_TIME: f32 = 5.0;
const DEFAULT_FALL_SPEED: f32 = 10.0;
const CHAR_WIDTH: f32 = 9.0;

#[derive(Debug)]
enum GameState {
    Stopped,
    Started,
    Paused,
}

#[derive(Debug)]
struct Item {
    pos: Vec2,
    word: String,
}

#[derive(AppState, Debug)]
struct Game {
    state: GameState,
    font: Font,
    stage: u32,
    time: f32,
    item_list: Vec<Item>,
    fall_speed: f32,
    rng: ThreadRng,
    size: (u32, u32),
    buffer: Vec<char>,
    cursor: usize,
}

impl Game {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
            .unwrap();
        Game {
            state: GameState::Stopped,
            font,
            stage: 0,
            time: DEFAULT_TIME,
            fall_speed: DEFAULT_FALL_SPEED,
            item_list: Vec::with_capacity(30),
            rng: thread_rng(),
            size: gfx.size(),
            buffer: Vec::new(),
            cursor: 0,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().set_size(800, 600);
    notan::init_with(Game::new)
        .add_config(win_config)
        .update(update)
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}

fn delete_char(game: &mut Game) {
    game.cursor -= 1;
    game.buffer.remove(game.cursor);
}

fn append_char(game: &mut Game, c: char) {
    game.buffer.insert(game.cursor, c);
    game.cursor += 1;
}

fn update(app: &mut App, game: &mut Game) {
    if app.keyboard.is_down(KeyCode::Escape) {
        app.exit();
    }

    match game.state {
        GameState::Stopped => {
            if app.keyboard.is_down(KeyCode::Space) {
                game.state = GameState::Started;
            }
        }
        GameState::Started => {
            for key in &app.keyboard.pressed {
                match key {
                    KeyCode::Return => {}
                    KeyCode::Left => game.cursor -= 1,
                    KeyCode::Up => (),
                    KeyCode::Right => game.cursor += 1,
                    KeyCode::Down => (),
                    KeyCode::Back => delete_char(game),
                    KeyCode::Space => append_char(game, ' '),
                    KeyCode::Key1 => append_char(game, '1'),
                    KeyCode::Key2 => append_char(game, '2'),
                    KeyCode::Key3 => append_char(game, '3'),
                    KeyCode::Key4 => append_char(game, '4'),
                    KeyCode::Key5 => append_char(game, '5'),
                    KeyCode::Key6 => append_char(game, '6'),
                    KeyCode::Key7 => append_char(game, '7'),
                    KeyCode::Key8 => append_char(game, '8'),
                    KeyCode::Key9 => append_char(game, '9'),
                    KeyCode::Key0 => append_char(game, '0'),
                    KeyCode::Numpad1 => append_char(game, '1'),
                    KeyCode::Numpad2 => append_char(game, '2'),
                    KeyCode::Numpad3 => append_char(game, '3'),
                    KeyCode::Numpad4 => append_char(game, '4'),
                    KeyCode::Numpad5 => append_char(game, '5'),
                    KeyCode::Numpad6 => append_char(game, '6'),
                    KeyCode::Numpad7 => append_char(game, '7'),
                    KeyCode::Numpad8 => append_char(game, '8'),
                    KeyCode::Numpad9 => append_char(game, '9'),
                    KeyCode::Numpad0 => append_char(game, '0'),
                    KeyCode::A => append_char(game, 'a'),
                    KeyCode::B => append_char(game, 'b'),
                    KeyCode::C => append_char(game, 'c'),
                    KeyCode::D => append_char(game, 'd'),
                    KeyCode::E => append_char(game, 'e'),
                    KeyCode::F => append_char(game, 'f'),
                    KeyCode::G => append_char(game, 'g'),
                    KeyCode::H => append_char(game, 'h'),
                    KeyCode::I => append_char(game, 'i'),
                    KeyCode::J => append_char(game, 'j'),
                    KeyCode::K => append_char(game, 'k'),
                    KeyCode::L => append_char(game, 'l'),
                    KeyCode::M => append_char(game, 'm'),
                    KeyCode::N => append_char(game, 'n'),
                    KeyCode::O => append_char(game, 'o'),
                    KeyCode::P => append_char(game, 'p'),
                    KeyCode::Q => append_char(game, 'q'),
                    KeyCode::R => append_char(game, 'r'),
                    KeyCode::S => append_char(game, 's'),
                    KeyCode::T => append_char(game, 't'),
                    KeyCode::U => append_char(game, 'u'),
                    KeyCode::V => append_char(game, 'v'),
                    KeyCode::W => append_char(game, 'w'),
                    KeyCode::X => append_char(game, 'x'),
                    KeyCode::Y => append_char(game, 'y'),
                    KeyCode::Z => append_char(game, 'z'),
                    _ => (),
                }
            }

            game.cursor = game.cursor.clamp(0, game.buffer.len());

            let delta = app.timer.delta_f32();

            for item in &mut game.item_list {
                item.pos.y += delta * game.fall_speed;
            }

            game.time -= delta;
            // println!("Time: {}", game.time);

            if game.time < 0.0 {
                game.item_list.push(Item {
                    pos: Vec2::new(game.rng.gen_range(10.0..game.size.0 as f32 - 30.0), 0.0),
                    word: "asdf".to_string(),
                });
                game.time = DEFAULT_TIME;
            }
        }
        GameState::Paused => {}
    }
}

fn draw(gfx: &mut Graphics, game: &mut Game) {
    game.size = gfx.size();
    let mut draw = gfx.create_draw();
    draw.clear(Color::WHITE);

    game.size = gfx.size();

    match game.state {
        GameState::Stopped => {
            let (width, height) = (game.size.0 as f32, game.size.1 as f32);
            draw.text(&game.font, "Press space to start!")
                .position(width / 2.0, height / 2.0)
                .color(Color::BLACK)
                .size(20.0);
        }
        GameState::Started => {
            for item in &game.item_list {
                draw.text(&game.font, &item.word)
                    .position(item.pos.x, item.pos.y)
                    .size(20.0)
                    .color(Color::BLACK);
            }

            draw.text(&game.font, &game.buffer.iter().collect::<String>())
                .position(
                    (game.size.0 as f32 / 2.0) - CHAR_WIDTH * (game.buffer.len() as f32 / 2.0),
                    game.size.1 as f32 - 30.0,
                )
                .size(20.0)
                .color(Color::BLACK);
        }
        GameState::Paused => {}
    }

    gfx.render(&draw);
}
