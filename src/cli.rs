mod minesweeper;
use read_input::prelude::*;
use pancurses::{initscr, endwin};


pub struct CommandLineInterface
{
    game:minesweeper::Minesweeper,
    cursor:(usize, usize)
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
        let cli = CommandLineInterface{
            game:minesweeper::Minesweeper::new(xdim, ydim, mines),
            cursor:(0, 0)
        };
        cli.display();
        cli
    }
    pub fn display(&self)
    {
        let board = self.game.get_board();
        for y in 0..board[0].len()
        {
            for x in 0..board.len()
            {
                if (x, y) == self.cursor
                {
                    print!("□");
                    continue;
                }
                match board[x][y]
                {
                    minesweeper::Tile::Clear(minesweeper::State::Hidden) => {
                        print!("~");
                    },
                    minesweeper::Tile::Clear(minesweeper::State::Near(mines)) => {
                        print!("{}", mines);
                    },
                    minesweeper::Tile::Mined => {
                        print!("~");
                    },
                    minesweeper::Tile::Flagged(_) => {
                        print!("F");
                    },
                }
            }
            println!("");
        }
    }

}