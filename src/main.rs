extern crate core;
extern crate sfml;

use core::slice;
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

const WIN_SIZE: (u32, u32) = (800, 600);
const WIN_TITLE: &str = "collision-detect";
const MOVE_STEP: f32 = 20.;

fn main() {
    println!("Collision Detection Test\n\nUse arrows to move the first character, \
              WASD for the other");

    let mut win = create_window();

    let tex0 = Texture::from_file("media/tex0.png").unwrap();
    let tex1 = Texture::from_file("media/tex1.png").unwrap();

    let mut manager = Manager::new();
    manager.add(Body::new(&tex0));
    manager.add(Body::with_pos(&tex1, (500., 400.)));

    'game: loop {
        win.clear(&Color::BLACK);

        for i in &manager {
            win.draw(i);
        }

        win.display();

        if manager.check_zeroth_others_rectangle() {
            println!("Simple Rectangle Collision!")
        } else {
            println!("No Collision")
        }

        let step = if Key::LShift.is_pressed() || Key::RShift.is_pressed(){
            if Key::Z.is_pressed() {
                MOVE_STEP / 16.
            } else {
                MOVE_STEP / 4.
            }
        } else {
            MOVE_STEP
        };

        while let Some(ev) = win.poll_event() {
            match ev {
                Event::KeyPressed { code, .. } => match code {
                    Key::Up    => manager.get_mut(0).move_(( 0.,        -step)),
                    Key::Down  => manager.get_mut(0).move_(( 0.,         step)),
                    Key::Left  => manager.get_mut(0).move_((-step,  0.)),
                    Key::Right => manager.get_mut(0).move_(( step,  0.)),

                    Key::W     => manager.get_mut(1).move_(( 0.,        -step)),
                    Key::S     => manager.get_mut(1).move_(( 0.,         step)),
                    Key::A     => manager.get_mut(1).move_((-step,  0.)),
                    Key::D     => manager.get_mut(1).move_(( step,  0.)),

                    Key::Escape => break 'game,
                    _ => {},
                }
				Event::Closed => break 'game,
                _ => {},
            }
        }
    }
}

fn create_window() -> RenderWindow {
    let settings = ContextSettings {
        antialiasing_level: 8,
        ..Default::default()
    };

    RenderWindow::new(VideoMode::new(WIN_SIZE.0, WIN_SIZE.1, 32), WIN_TITLE,
            Default::default(), &settings)
}

struct Body<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Body<'s> {
    fn new(tex: &'s TextureRef) -> Body {
        Body {
            sprite: Sprite::with_texture(tex),
        }
    }

    fn with_pos<P: Into<Vector2f>>(tex: &'s TextureRef, pos: P) -> Body {
        let mut sprite = Sprite::with_texture(tex);
        sprite.set_position(pos);

        Body {
            sprite,
        }
    }

    fn global_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }
}

impl<'s> Drawable for Body<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw_sprite(&self.sprite, states)
    }
}

impl<'s> Transformable for Body<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        self.sprite.set_position(position)
    }

    fn set_rotation(&mut self, angle: f32) {
        self.sprite.set_rotation(angle)
    }

    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        self.sprite.set_scale(scale)
    }

    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.sprite.set_origin(origin)
    }

    fn position(&self) -> Vector2f {
        self.sprite.position()
    }

    fn rotation(&self) -> f32 {
        self.sprite.rotation()
    }

    fn get_scale(&self) -> Vector2f {
        self.sprite.get_scale()
    }

    fn origin(&self) -> Vector2f {
        self.sprite.origin()
    }

    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.sprite.move_(offset)
    }

    fn rotate(&mut self, angle: f32) {
        self.sprite.rotate(angle)
    }

    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        self.sprite.scale(factors)
    }

    fn transform(&self) -> Transform {
        self.sprite.transform()
    }

    fn inverse_transform(&self) -> Transform {
        self.sprite.inverse_transform()
    }
}

struct Manager<'s> {
    bodies: Vec<Body<'s>>,
}

impl<'s> Manager<'s> {
    fn new() -> Manager<'s> {
        Manager {
            bodies: Vec::new(),
        }
    }

    fn add(&mut self, body: Body<'s>) {
        self.bodies.push(body)
    }

    //fn get(&self, idx: usize) -> &Body<'s> {
    //    &self.bodies[idx]
    //}

    fn get_mut(&mut self, idx: usize) -> &mut Body<'s> {
        &mut self.bodies[idx]
    }

    /// Check for a collision between the zeroth and the others.
    fn check_zeroth_others_rectangle(&self) -> bool {
        assert_ne!(self.bodies.len(), 0, "Need at least 1 body");

        let zeroth = self.bodies[0].global_bounds();

        for (idx, other) in self.bodies.iter().enumerate() {
            if idx == 0 {
                continue
            }

            if other.global_bounds().intersection(&zeroth).is_some() {
                return true
            }
        }

        false
    }
}

impl<'a, 's> IntoIterator for &'a Manager<'s> {
    type Item = &'a Body<'s>;
    type IntoIter = slice::Iter<'a, Body<'s>>;

    fn into_iter(self) -> Self::IntoIter {
        self.bodies.iter()
    }
}
