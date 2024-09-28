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
pub struct Point {}

impl Point {
    pub fn new(width: i32, height: i32) -> Point {
        todo!()
    }
    pub fn random(width: i32, height: i32) -> Point {
        todo!()
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {}
}





//--------------------------------------------------------------------------------
// Rectangle
//--------------------------------------------------------------------------------
pub struct Rectangle {}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        todo!()
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {}
}




//--------------------------------------------------------------------------------
// Circle
//--------------------------------------------------------------------------------
pub struct Circle {}

impl Circle {
    pub fn new() {}
    pub fn random(width: i32, height: i32) -> Circle {
        todo!()
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {}
}





//--------------------------------------------------------------------------------
// Line
//--------------------------------------------------------------------------------
pub struct Line {}

impl Line {
    pub fn new() {}
    pub fn random(width: i32, height: i32) -> Line {
        todo!()
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {}
}



//--------------------------------------------------------------------------------
// Triangle
//--------------------------------------------------------------------------------
pub struct Triangle {}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) ->Triangle {
        todo!()
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