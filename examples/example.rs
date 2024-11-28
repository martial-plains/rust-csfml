use rust_sfml::{
    audio::Music,
    graphics::{color::Color, texture::Texture, Font, RenderWindow, Sprite, Text},
    system::Vector2f,
    types::Result,
    window::{Event, Style, VideoMode},
};

fn main() -> Result<()> {
    let mode = VideoMode {
        width: 800,
        height: 600,
        bits_per_pixel: 32,
    };

    let mut window = RenderWindow::create(mode, "SFML window", Style::RESIZE | Style::CLOSE, None)?;

    let texture = Texture::create_from_file("sys/examples/sfml_logo.png")?;
    let mut sprite = Sprite::create()?;
    sprite.set_texture(Some(&texture));
    let sprite_position = Vector2f { x: 200.0, y: 200.0 };
    sprite.set_position(sprite_position);

    let font = Font::create_from_file("sys/examples/tuffy.ttf")?;
    let mut text = Text::create()?;
    text.set_string("Hello, SFML");
    text.set_font(&font);
    text.set_character_size(50);

    let music = Music::create_from_file("sys/examples/doodle_pop.ogg");
    music.play();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {
                    window.clear(Color::BLACK);
                    window.draw(&sprite, None);
                    window.draw(&text, None);
                    window.display();
                }
            }
        }
    }

    Ok(())
}
