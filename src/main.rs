mod minesweeper;
fn main()
{
    let mut game = minesweeper::Minesweeper::new(10,10,10);
    game.display_cleared();
}