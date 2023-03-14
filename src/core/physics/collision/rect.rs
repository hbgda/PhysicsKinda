use sdl2::rect::Rect;

use super::line::{Line, line_intersect, Point};

pub fn collide_top(rect: Rect, line: Line) -> Option<i32> {
    let top_line = Line(
        Point { x: rect.x, y: rect.y },
        Point { x: rect.x + rect.width() as i32,
                y: rect.y }
    );
    if let Some((x, _)) = line_intersect(top_line, line) {
        return Some(x);
    }
    None
}

pub fn collide_left(rect: Rect, line: Line) -> Option<i32> {
    let left_line = Line(
        Point { x: rect.x, y: rect.y },
        Point { x: rect.x,
                y: rect.y + rect.height() as i32 }
    );
    if let Some((_, y)) = line_intersect(left_line, line) {
        return Some(y);
    }
    None
}

pub fn collide_right(rect: Rect, line: Line) -> Option<i32> {
    None
}

pub fn collide_bottom(rect: Rect, line: Line) -> Option<i32> {
    None
}