#[derive(Clone, Copy)]
pub enum TileValue {
    Mine,
    Empty(u32),
}

#[derive(Clone, Copy)]
pub enum TileState {
    Closed(TileMarking),
    Opened,
}

#[derive(Clone, Copy)]
pub enum TileMarking {
    None,
    Flag,
    Question,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub value: TileValue,

    pub state: TileState,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            value: TileValue::Empty(0),
            state: TileState::Closed(TileMarking::None),
        }
    }

    pub fn set_mine(&mut self) {
        self.value = TileValue::Mine
    }

    pub fn inc_mine_count(&mut self) {
        if let TileValue::Empty(n) = self.value {
            self.value = TileValue::Empty(n + 1)
        }
    }

    /// Opens a tile if yet unopen
    ///
    /// Will return a tile's `TileValue` if the tile was closed when `open` was
    /// called.
    pub fn open(&mut self) -> Option<TileValue> {
        if let TileState::Closed(_) = self.state {
            self.state = TileState::Opened;
            return Some(self.value);
        }

        None
    }

    pub fn cycle(&mut self) {
        if let TileState::Closed(mark) = self.state {
            self.state = TileState::Closed(match mark {
                TileMarking::None => TileMarking::Flag,
                TileMarking::Flag => TileMarking::Question,
                TileMarking::Question => TileMarking::None,
            });
        }
    }
}
