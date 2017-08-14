use sdl2;
use super::TILE;

pub struct Snake {
    velocity: u8, // Tiles per move_forward()
    color: sdl2::pixels::Color,
    direction: Facing,
    tail_len: usize,
    head: sdl2::rect::Rect,
    tail: Vec<sdl2::rect::Rect>
}

#[derive(PartialEq)]
pub enum Facing {
    Top,
    Bottom,
    Left,
    Right
}

impl Facing {
    pub fn opposite(&self) -> Self {
        match *self {
            Facing::Top    => Facing::Bottom,
            Facing::Bottom => Facing::Top,
            Facing::Left   => Facing::Right,
            Facing::Right  => Facing::Left
        }
    }
}

impl Snake {
    pub fn new(v: u8, c: sdl2::pixels::Color) -> Self {
        Self {
            velocity: v,
            color: c,
            tail_len: 5,
            direction: Facing::Right,
            head: sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32),
            tail: vec![
                sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32),
                sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32),
                sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32),
                sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32),
                sdl2::rect::Rect::new(0, 0, TILE as u32, TILE as u32)
            ]
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.head);

        for r in self.tail.clone() {
            canvas.fill_rect(r);
        }
    }

    pub fn head(&self) -> &sdl2::rect::Rect {
        &self.head
    }

    // TODO: Name method 'better'
    pub fn look_to(&mut self, direction: Facing) {
        if self.direction != direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn move_forward(&mut self) {
        let (x, y) = (self.head.x(), self.head.y());

        self.move_tail();

        match self.direction {
            Facing::Top => {
                self.head.set_x(x);
                self.head.set_y(y - (TILE * self.velocity) as i32);
            },
            Facing::Bottom => {
                self.head.set_x(x);
                self.head.set_y(y + (TILE * self.velocity) as i32);
            },
            Facing::Left => {
                self.head.set_x(x - (TILE * self.velocity) as i32);
                self.head.set_y(y);
            },
            Facing::Right => {
                self.head.set_x(x + (TILE * self.velocity) as i32);
                self.head.set_y(y);
            }
        }
    }

    pub fn tail_grow(&mut self) {
        self.tail_grow_by(1)
    }

    pub fn tail_grow_by(&mut self, len: usize) {
        self.tail_len += len;
    }

    // Take the last element of Tail, since it will be discarded and clone head(x, y) to
    // last_element(x, y) and put it on the first place of Vec.
    // All other elements will be already shifted after the insertion.
    fn move_tail(&mut self) {
        // If there is less items on tail than the current tail length, then duplicate the last one
        // to be moved instead
        if self.tail_len != self.tail.len() {
            let dup = self.tail.iter().last().unwrap().clone();
            self.tail.push(dup);
        }

        match self.tail.pop() {
            Some(mut rec) => {
                rec.set_x(self.head.x());
                rec.set_y(self.head.y());
                self.tail.insert(0, rec);
            },
            None => ()
        }
    }
}

