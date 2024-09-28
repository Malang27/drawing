use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image) {}
    fn color() {}
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color) {}
}

//--------------------------------------------------------------------------------
// Point
//--------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(pos_x: i32, pos_y: i32) -> Point {
        Point { x: pos_x, y: pos_y }
    }
    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {}
}

//--------------------------------------------------------------------------------
// Rectangle
//--------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            top_left: p1.clone(),
            bottom_right: p2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {}
}

//--------------------------------------------------------------------------------
// Circle
//--------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Circle {
    c: Point,
    r: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            c: center.clone(),
            r: radius,
        }
    }
    pub fn random(width: i32, height: i32) -> Circle {
        Circle {
            c: Point::random(width, height),
            r: rand::thread_rng().gen_range(1..width),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {}
}

//--------------------------------------------------------------------------------
// Line
//--------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { p1: start, p2: end }
    }

    pub fn random(width: i32, height: i32) -> Line {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {}
}

//--------------------------------------------------------------------------------
// Triangle
//--------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p_1: &Point, p_2: &Point, p_3: &Point) -> Triangle {
        Triangle {
            p1: p_1.clone(),
            p2: p_2.clone(),
            p3: p_3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {}
}

//--------------------------------------------------------------------------------
// Image
//--------------------------------------------------------------------------------
impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

//--------------------------------------------------------------------------------
// Image
//--------------------------------------------------------------------------------
pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color {
        r: rng.gen_range(1..=255),
        g: rng.gen_range(1..=255),
        b: rng.gen_range(1..=255),
        a: 255,
    }
}
