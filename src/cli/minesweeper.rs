use rand::prelude::*;

#[derive(Clone)]
pub enum State
{
    Hidden,
    Near(usize),
}

#[derive(Clone)]
pub enum Tile
{
    Clear(State),
    Mined,
    Flagged(Box<Tile>)
}

pub struct Minesweeper
{
    board:Vec<Vec<Tile>>
}

impl Minesweeper
{
    pub fn new(xdim:usize, ydim:usize, mines:usize) -> Minesweeper
    {
        let mut game = Minesweeper{board:Vec::new()};
        game.generate(xdim, ydim, mines);
        game
    }

    pub fn touch_mine(&mut self, x:usize, y:usize) -> bool
    {
        match self.board[x][y]
        {
            Tile::Clear(State::Hidden) => {
                let mines = self.find_mines_around(x, y);
                self.board[x][y] = Tile::Clear(State::Near(mines));
                if mines == 0
                {
                    for dx in -1..2
                    {
                        if x as isize + dx == -1
                        || x as isize + dx == self.board.len() as isize
                        {
                            continue;
                        }
                        for dy in -1..2
                        {
                            if y as isize + dy == -1
                            || y as isize + dy == self.board[0].len() as isize
                            || dx == dy
                            || dx * -1 == dy
                            {
                                continue;
                            }
                            match &self.board[(x as isize + dx) as usize][(y as isize + dy) as usize]
                            {
                                Tile::Clear(State::Hidden) => {
                                    self.touch_mine((x as isize + dx) as usize, (y as isize + dy) as usize);
                                },
                                _ => {}, // do nothing
                            }
                        }
                    }
                }
                false
            },
            Tile::Clear(_) => {false},
            Tile::Flagged(_) => {false},
            Tile::Mined => {true},
        }
    }

    pub fn toggle_flag(&mut self, x:usize, y:usize) -> bool
    {
        match &self.board[x][y]
        {
            Tile::Flagged(tile) => {
                self.board[x][y] = *tile.clone();
                true
            },
            Tile::Mined => {
                self.board[x][y] = Tile::Flagged(Box::new(Tile::Mined));
                true
            },
            Tile::Clear(State::Hidden) => {
                self.board[x][y] = Tile::Flagged(Box::new(Tile::Clear(State::Hidden)));
                true
            }
            _ => {false},
        }
    }

    pub fn get_board(&self) -> Vec<Vec<Tile>>
    {
        return self.board.clone();
    }

    fn generate(&mut self, xdim:usize, ydim:usize, mut mines:usize)
    {
        let mut tiles = Vec::new();
        for _ in 0..(xdim * ydim)
        {
            if mines == 0
            {
                tiles.push(Tile::Clear(State::Hidden));
            }
            else
            {
                tiles.push(Tile::Mined);
                mines -= 1;
            }
        }
        let mut rng = rand::thread_rng();
        tiles.shuffle(&mut rng);
        for _x in 0..xdim
        {
            let mut column = Vec::new();
            for _y in 0..ydim
            {
                match tiles.pop()
                {
                    Some(tile) => {
                        column.push(tile)
                    },
                    None => {
                        column.push(Tile::Mined);
                        println!("Error in generation!");
                    },
                }
            }
            self.board.push(column);
        }
    }


    fn find_mines_around(&self, x:usize, y:usize) -> usize
    {
        let mut mines:usize = 0;
        for dx in -1..2
        {
            if x as isize + dx == -1
            || x as isize + dx == self.board.len() as isize
            {
                continue;
            }
            for dy in -1..2
            {
                if y as isize + dy == -1
                || y as isize + dy == self.board[0].len() as isize
                {
                    continue;
                }
                if self.is_mined((x as isize + dx) as usize, (y as isize + dy) as usize)
                {
                    mines += 1;
                }
            }
        }
        mines
    }

    fn is_mined(&self, x:usize, y:usize) -> bool
    {
        match &self.board[x][y]
        {
            Tile::Mined => {true},
            Tile::Flagged(tile) => {
                match **tile
                {
                    Tile::Mined => {true},
                    _ => {false},
                }
            },
            _ => {false},
        }
    }
}
