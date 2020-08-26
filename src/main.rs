mod cli;

fn main()
{
    match cli::CommandLineInterface::new()
    {
        Some(mut run) => {
            run.event_loop()
        },
        None => {
            println!("Error!");
        },
    }
}