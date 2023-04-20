#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone, Copy)]
pub struct Line(pub Point, pub Point);

pub fn line_intersect(a: Line, b: Line) -> Option<(i32, i32)> {
    // dbg!(a, b);
    let a1 = a.1.y - a.0.y;
    let b1 = a.0.x - a.1.x;
    let c1 = a1 * a.0.x + b1 * a.0.y;
    let a2 = b.1.y - b.0.y;
    let b2 = b.0.x - b.1.x;
    let c2 = a2 * b.0.x + b2 * b.0.y;
    let denom = a1 * b2 - a2 * b1;

    if denom == 0 {
        return None;
    }

    let intersect_x = (b2 * c1 - b1 * c2) / denom;
    let intersect_y = (a1 * c2 - a2 * c1) / denom;
    let ratio_ax = (intersect_x - a.0.x) as f32 / (a.1.x - a.0.x) as f32;
    let ratio_ay = (intersect_y - a.0.y) as f32 / (a.1.y - a.0.y) as f32;
    let ratio_bx = (intersect_x - b.0.x) as f32 / (b.1.x - b.0.x) as f32;
    let ratio_by = (intersect_y - b.0.y) as f32 / (b.1.y - b.0.y) as f32;

    // println!("{denom}");
    // println!("X: {}, Y: {}, X1: {}, Y1: {}", a.0.x, a.0.y, a.1.x, a.1.y);
    // println!("X: {}, Y: {}, X1: {}, Y1: {}", b.0.x, b.0.y, b.1.x, b.1.y);
    // println!("A1: {a1} B1: {b1} C1: {c1}");
    // println!("A2: {a2} B2: {b2} C2: {c2}");

    if ((ratio_ax >= 0.0 && ratio_ax <= 1.0) || (ratio_ay >= 0.0 && ratio_ay <= 1.0))
    && ((ratio_bx >= 0.0 && ratio_bx <= 1.0) || (ratio_by >= 0.0 && ratio_by <= 1.0)) {
        return Some((intersect_x, intersect_y));
    }
    None
}

impl Line {
    pub fn left_of()
}

pub fn lines_contain_point(left: Line, right: Line, point: Point) -> bool {
    
}
