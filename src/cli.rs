mod minesweeper;
use read_input::prelude::*;
use easycurses::*;

enum Choice
{
    Up,
    Down,
    Left,
    Right,
    Escape,
    Flag,
    Touch,
    Invalid,
}

pub struct CommandLineInterface
{
    game:minesweeper::Minesweeper,
    cursor:(usize, usize),
    window:easycurses::EasyCurses,
}

impl CommandLineInterface
{
    pub fn new() -> Option<Self>
    {
        println!("Enter x dimension:");
        let xdim = input().get();
        println!("Enter y dimension:");
        let ydim = input().get();
        println!("Enter number of mines:");
        let mines = input().get();
        match easycurses::EasyCurses::initialize_system()
        {
            Some(ec) => {
                let cli = CommandLineInterface{
                    game:minesweeper::Minesweeper::new(xdim, ydim, mines),
                    cursor:(0, 0),
                    window:ec,
                };
                Some(cli)
            },
            None => {
                None
            }
        }
    }

    pub fn event_loop(&mut self)
    {
        let xdim = self.game.get_board().len();
        let ydim = self.game.get_board().len();
        loop
        {
            self.display();
            match self.get_input()
            {
                Choice::Up => {
                    if self.cursor.1 + 1 != ydim
                    {
                        self.cursor.1 += 1;
                    }
                },
                Choice::Left => {
                    if self.cursor.0 as isize - 1 != -1
                    {
                        self.cursor.0 -= 1;
                    }
                },
                Choice::Down => {
                    if self.cursor.1 as isize - 1 != -1
                    {
                        self.cursor.1 -= 1;
                    }
                },
                Choice::Right => {
                    if self.cursor.0 + 1 != xdim
                    {
                        self.cursor.0 += 1;
                    }
                },
                Choice::Flag => {
                    if !self.game.toggle_flag(self.cursor.0, self.cursor.1)
                    {
                        println!("Invalid position for flag");
                    }
                },
                Choice::Touch => {
                    if self.game.touch_mine(self.cursor.0, self.cursor.1)
                    {
                        break;
                    }
                },
                Choice::Escape => {
                    break;
                },
                Choice::Invalid => {},
            }
        }
    }

    fn display(&mut self)
    {
        self.window.clear();
        let board = self.game.get_board();
        for y in (0..board[0].len()).rev()
        {
            for x in 0..board.len()
            {
                if (x, y) == self.cursor
                {
                    self.window.print("X");
                    continue;
                }
                match board[x][y]
                {
                    minesweeper::Tile::Clear(minesweeper::State::Hidden) => {
                        self.window.print("~");
                    },
                    minesweeper::Tile::Clear(minesweeper::State::Near(mines)) => {
                        self.window.print(format!("{}", mines));
                    },
                    minesweeper::Tile::Mined => {
                        self.window.print("~");
                    },
                    minesweeper::Tile::Flagged(_) => {
                        self.window.print("F");
                    },
                }
            }
            self.window.print("\n");
        }
     self.window.refresh();
    }

    fn get_input(&mut self) -> Choice
    {
        match self.window.get_input()
        {
            Some(Input::Character(c)) => {
                match c
                {
                    'w' => {
                        Choice::Up
                    },
                    'a' => {
                        Choice::Left
                    },
                    's' => {
                        Choice::Down
                    },
                    'd' => {
                        Choice::Right
                    },
                    'f' => {
                        Choice::Flag
                    },
                    '\n' => {
                        Choice::Touch
                    },
                    'e' => {
                        Choice::Touch
                    },
                    'Q' => {
                        Choice::Escape
                    },
                    _ => {
                        Choice::Invalid
                    },
                }
            },
            Some(_) => {
                Choice::Invalid
            },
            None => {
                Choice::Invalid
            },
        }
    }
}