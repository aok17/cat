use std::env;
mod cli;
mod handler;


fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let parsed = cli::parse(args);
    match parsed {
        Ok(parsed_args) => {let result = handler::run(cli::get_filepaths(parsed_args)); print!("{}",result)},
        Err(cli::Error::Help(msg)) => print!("{}",msg),
        Err(cli::Error::NoArgs(msg)) => print!("{}",msg),
        Err(cli::Error::InvalidOpts(msg)) => print!("{}",msg)
    }
}
