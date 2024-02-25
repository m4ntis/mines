#[derive(Clone, Copy)]
pub enum Value {
    Mine,
    Empty(u32),
}

#[derive(Clone, Copy)]
pub enum State {
    Closed(Marking),
    Opened,
}

#[derive(Clone, Copy)]
pub enum Marking {
    None,
    Flag,
    Question,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub value: Value,

    pub state: State,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            value: Value::Empty(0),
            state: State::Closed(Marking::None),
        }
    }

    pub fn set_mine(&mut self) {
        self.value = Value::Mine
    }

    pub fn inc_mine_count(&mut self) {
        if let Value::Empty(n) = self.value {
            self.value = Value::Empty(n + 1)
        }
    }

    /// Opens a tile if yet unopen
    ///
    /// Will return a tile's `Value` if the tile was closed when `open` was
    /// called.
    pub fn open(&mut self) -> Option<Value> {
        if let State::Closed(_) = self.state {
            self.state = State::Opened;
            return Some(self.value);
        }

        None
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self.state {
            State::Opened => match self.value {
                Value::Mine => "*".to_string(),
                Value::Empty(n) => {
                    if n == 0 {
                        " ".to_string()
                    } else {
                        n.to_string()
                    }
                }
            },
            State::Closed(mark) => match mark {
                Marking::None => "⎕".to_string(),
                Marking::Flag => "⌻".to_string(),
                Marking::Question => "⍰".to_string(),
            },
        }
    }
}
