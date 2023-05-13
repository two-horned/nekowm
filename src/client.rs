use x11rb::protocol::xproto::Window;
use crate::geometry::Geometry;

impl Client {
    pub fn new(win: Window) -> Client {
        Client { win, now: Geometry::new(), old: Geometry::new(), fullscreen: false, floating: false, hidden: false, marked: false }
    }

    pub fn win(&self) -> Window {
        self.win
    }

    pub fn now(&self) -> &Geometry {
        &self.now
    }
    pub fn old(&self) -> &Geometry {
        &self.old
    }
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }
    pub fn floating(&self) -> bool {
        self.floating
    }
    pub fn hidden(&self) -> bool {
        self.hidden
    }
    pub fn marked(&self) -> bool {
        self.marked
    }

    pub fn age(&mut self) {
        self.old = self.now;
    }

    pub fn kill(&self) {
    }
}

pub struct Client {
    win: Window,
    now: Geometry,
    old: Geometry,
    fullscreen: bool,
    floating: bool,
    hidden: bool,
    marked: bool,
}
