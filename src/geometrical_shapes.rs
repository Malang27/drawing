use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image) {
        let _ = image;
    }
}

pub trait Displayable {
    fn display(&mut self, _x: i32, _y: i32, _color: &Color) {}
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
    fn draw(&self, image: &mut Image) {
        let color = random_color();
        image.display(self.x, self.y, &color);
    }
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
    fn draw(&self, image: &mut Image) {
        Line::new(
            self.top_left.clone(),
            Point::new(self.bottom_right.x, self.top_left.y),
        )
        .draw(image);
        Line::new(
            Point::new(self.bottom_right.x, self.top_left.y),
            self.bottom_right.clone(),
        )
        .draw(image);
        Line::new(
            self.bottom_right.clone(),
            Point::new(self.top_left.x, self.bottom_right.y),
        )
        .draw(image);
        Line::new(
            Point::new(self.top_left.x, self.bottom_right.y),
            self.top_left.clone(),
        )
        .draw(image);
    }
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
    fn draw(&self, image: &mut Image) {
        let color = random_color();
        let mut x = self.r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(self.c.x + x, self.c.y + y, &color);
            image.display(self.c.x + y, self.c.y + x, &color);
            image.display(self.c.x - y, self.c.y + x, &color);
            image.display(self.c.x - x, self.c.y + y, &color);
            image.display(self.c.x - x, self.c.y - y, &color);
            image.display(self.c.x - y, self.c.y - x, &color);
            image.display(self.c.x + y, self.c.y - x, &color);
            image.display(self.c.x + x, self.c.y - y, &color);

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }

            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
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
    fn draw(&self, image: &mut Image) {
        let mut x1 = self.p1.x;
        let mut y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;

        while x1 != x2 || y1 != y2 {
            let color = random_color();
            image.display(x1, y1, &color);
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x1 += sx;
            }
            if e2 < dy {
                err += dx;
                y1 += sy;
            }
        }
    }
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
    fn draw(&self, image: &mut Image) {
        Line::new(self.p1.clone(), self.p2.clone()).draw(image); // Side 1.
        Line::new(self.p2.clone(), self.p3.clone()).draw(image); // Side 2.
        Line::new(self.p3.clone(), self.p1.clone()).draw(image); // Side 3.
    }
}

//--------------------------------------------------------------------------------
// Image
//--------------------------------------------------------------------------------

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: &Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color.clone()).unwrap();
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
