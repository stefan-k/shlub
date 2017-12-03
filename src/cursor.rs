use ncurses::*;

pub struct Cursor {
    pub y: i32,
    pub x: i32,
    max_y: i32,
    max_x: i32,
}

fn get_screen_max() -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    (max_y, max_x)
}

impl Cursor {
    // pub fn new(y: i32, x: i32) -> Self {
    //     let (max_y, max_x) = get_screen_max();
    //     Cursor { y, x, max_y, max_x }
    // }

    pub fn current_pos() -> Self {
        let (max_y, max_x) = get_screen_max();
        let mut cx = 0;
        let mut cy = 0;
        getyx(stdscr(), &mut cy, &mut cx);
        Cursor {
            y: cy,
            x: cx,
            max_y: max_y,
            max_x: max_x,
        }
    }

    pub fn set(&mut self, y: i32, x: i32) -> &mut Self {
        self.update_max();
        self.y = y + (x - (x % self.max_x)) / self.max_x;
        self.x = x % self.max_x;
        self
    }

    pub fn update_max(&mut self) -> &mut Self {
        let (max_y, max_x) = get_screen_max();
        self.max_y = max_y;
        self.max_x = max_x;
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.update_max();
        if self.x > 0 {
            self.x -= 1;
        } else {
            self.x = self.max_x;
            self.up();
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.update_max();
        if self.x < self.max_x {
            self.x += 1;
        } else {
            self.x = 0;
            self.down();
        }
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.y -= 1;
        self
    }

    pub fn down(&mut self) -> &mut Self {
        if self.y + 1 > self.max_y - 1 {
            wscrl(stdscr(), 1);
            // why tho
            mv(self.max_y - 2, 0);
        }
        self.y += 1;
        self
    }
}
