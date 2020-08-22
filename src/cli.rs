mod minesweeper;
use read_input::prelude::*;


struct CommandLineInterface
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
        cli
    }
    pub fn display(&self)
    {
        for y in 0..self.game.get_board()[0].len()
        {
            for x in 0..self.game.get_board().len()
            {
                match self.game.get_board()[x][y]
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