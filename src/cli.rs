mod minesweeper;
use read_input::prelude::*;
use pancurses::{initscr, Input, noecho};

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
    window:pancurses::Window,
}

impl CommandLineInterface
{
    pub fn new() -> CommandLineInterface
    {
        println!("Enter x dimension:");
        let xdim = input().get();
        println!("Enter y dimension:");
        let ydim = input().get();
        println!("Enter number of mines:");
        let mines = input().get();
        noecho();
        let cli = CommandLineInterface{
            game:minesweeper::Minesweeper::new(xdim, ydim, mines),
            cursor:(0, 0),
            window:initscr(),
        };
        cli
    }

    pub fn event_loop(&mut self)
    {
        let xsize = self.game.get_board().len();
        let ysize = self.game.get_board().len();
        loop
        {
            self.display();
            match self.get_input()
            {
                Choice::Up => {
                    match &mut self.cursor
                    {
                        (_, ysize) => {},
                        (x, y) => {
                            *y += 1;
                        },
                    }
                },
                Choice::Left => {
                    match &mut self.cursor
                    {
                        (0, _) => {},
                        (x, y) => {
                            *x -= 1;
                        },
                    }
                },
                Choice::Down => {
                    match &mut self.cursor
                    {
                        (_, 0) => {},
                        (x, y) => {
                            *y -= 1;
                        },
                    }
                }
                Choice::Right => {
                    match &mut self.cursor
                    {
                        (xsize, _) => {},
                        (x, y) => {
                            *x += 1;
                        },
                    }
                }
                _ => {},
            }
        }
    }

    fn display(&self)
    {
        self.window.refresh();
        let board = self.game.get_board();
        for y in 0..board[0].len()
        {
            for x in 0..board.len()
            {
                if (x, y) == self.cursor
                {
                    self.window.printw("□");
                    continue;
                }
                match board[x][y]
                {
                    minesweeper::Tile::Clear(minesweeper::State::Hidden) => {
                        self.window.printw("~");
                    },
                    minesweeper::Tile::Clear(minesweeper::State::Near(mines)) => {
                        self.window.printw(format!("{}", mines));
                    },
                    minesweeper::Tile::Mined => {
                        self.window.printw("~");
                    },
                    minesweeper::Tile::Flagged(_) => {
                        self.window.printw("F");
                    },
                }
            }
            self.window.printw("\n");
        }
     self.window.refresh();
    }

    fn get_input(&self) -> Choice
    {
        self.window.keypad(true);
        match self.window.getch()
        {
            Some(Input::Character(c)) => {
                self.window.keypad(false);
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
            }
        }
    }
}