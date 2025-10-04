use std::thread;
use std::time::Duration;
use hyprland::{
    data::{Client, CursorPosition},
    dispatch::{Dispatch, DispatchType},
    shared::{HyprData, HyprDataActiveOptional},
};
use std::cmp;


const ADJUST: i64 = 12;


struct Position { x: i64, y: i64 }
struct Vector { x: i64, y: i64 }
struct Window { at: Position, size: Vector }
struct RadialCoords{ angle: f64, radius: f64 }

impl Position {
    fn from_cursor() -> Self {
        let cursor_pos = CursorPosition::get().unwrap();
        return Self { x: cursor_pos.x, y: cursor_pos.y }
    }
    fn snap_to_window(&self, window: &Window) -> Self {
        return Self {
            x: cmp::min(cmp::max(self.x, window.at.x + ADJUST), window.size.x + window.at.x - ADJUST),
            y: cmp::min(cmp::max(self.y, window.at.y + ADJUST), window.size.y + window.at.y - ADJUST)
        }
    }
    fn is_inside_window(&self, window: &Window) -> bool {
        return (self.x >= window.at.x + ADJUST) && (self.x <= window.at.x + window.size.x - ADJUST) &&
            (self.y >= window.at.y + ADJUST) && (self.y <= window.at.y + window.size.y - ADJUST);
    }
}

impl Vector {
    fn between(pos0: &Position, pos1: &Position) -> Self {
        return Self { x: pos1.x - pos0.x, y: pos1.y - pos0.y }
    }
    fn apply(&self, pos: &Position) -> Position {
        return Position { x: self.x + pos.x, y: self.y + pos.y }
    }
}

impl Window {
    fn get_active() -> Self {
        let client = Client::get_active().unwrap().unwrap();
        Self {
            at: Position { x: client.at.0 as i64, y: client.at.1 as i64 },
            size: Vector { x: client.size.0 as i64, y: client.size.1 as i64 }
        }
    }
    fn center(&self) -> Position {
        return Position { x: self.at.x + self.size.x / 2, y: self.at.y + self.size.y / 2 }
    }
}

impl RadialCoords {
    fn from_cursor(cursor_pos: &Position, window: &Window) -> Self {
        let center = window.center();
        let center_offset = Vector::between(&center, &cursor_pos);
        let x = center_offset.x as f64;
        let y = center_offset.y as f64;
        let adj: f64 = if x < 0. { std::f64::consts::PI } else { 0. };
        return Self {
            angle: f64::atan(y / x) + adj,
            radius: f64::sqrt(x.powf(2.) + y.powf(2.))
        }
    }
    fn reverse_angle(&mut self) -> () { self.angle += std::f64::consts::PI; }
    fn to_position(&self, window: &Window) -> Position {
        let center_offset = Vector {
            x: (self.radius * f64::cos(self.angle)) as i64,
            y: (self.radius * f64::sin(self.angle)) as i64
        };
        let center = window.center();
        let pos = center_offset.apply(&center);
        return pos.snap_to_window(window);
    }
}


fn main() {
    let sleep_time = 1;

    let window = Window::get_active();

    loop {
        let cursor = Position::from_cursor();
        if !cursor.is_inside_window(&window) {
            let mut radial = RadialCoords::from_cursor(&cursor, &window);
            println!("{} {}", radial.angle, radial.radius);
            radial.reverse_angle();
            let new_pos = radial.to_position(&window);
            Dispatch::call(DispatchType::MoveCursor(new_pos.x, new_pos.y)).unwrap();
        }
        thread::sleep(Duration::from_millis(sleep_time));
    }
}
