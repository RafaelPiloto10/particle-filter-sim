use std::{rc::Rc, cell::RefCell};

pub struct Pos {
    pub x: i32,
    pub y: i32
}

// Defines a position
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        return Pos {
            x,
            y 
        }
    }
}

// Defines an agent such as a particle or robot
pub struct Agent {
    pub pos: Rc<RefCell<Pos>>
}

impl Agent {
    pub fn new(x: i32, y: i32) -> Self {
        return Agent {
            pos: Rc::from(RefCell::from(Pos::new(x, y))),
        }
    }

    pub fn update_pos(&self, x: i32, y: i32) {
        let mut pos = self.pos.borrow_mut();
        pos.x = x;
        pos.y = y;
    }
}
