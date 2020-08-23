mod cli;
use pancurses::endwin;

fn main()
{
    let mut run = cli::CommandLineInterface::new();

    run.event_loop();

    endwin();
}