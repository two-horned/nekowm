impl Geometry {
    pub fn new() -> Geometry {
        Geometry { x: 0, y: 0, w: 1, h: 1 }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn w(&self) -> i32 {
        self.w
    }

    pub fn h(&self) -> i32 {
        self.h
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_w(&mut self, w: i32) {
        self.w = w;
    }

    pub fn set_h(&mut self, h: i32) {
        self.h = h;
    }
}

#[derive(Clone, Copy)]
pub struct Geometry {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}
