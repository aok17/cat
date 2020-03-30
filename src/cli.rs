pub struct Config {
    filepaths: Vec<String>
}

pub fn make_config(filepaths: Vec<String>) -> Config{
    return Config{filepaths}
}

pub fn get_filepaths(config: Config) -> Vec<String> {
    return config.filepaths
}

pub enum Error {
    Help(String),
    InvalidOpts(String),
    NoArgs(String)
}

const NOARGSSTR: &str = "Error: No arguments provided";
const HELPSTR: &str = "";


pub fn parse(args:Vec<String>) -> Result<Config,Error>{
        let (_,tail) = args.split_at(1);
        if tail.is_empty() { return Err(Error::NoArgs(String::from(NOARGSSTR)))}
        else if tail.contains(&String::from("-h")) || tail.contains(&String::from("--help")) {
            return Err(Error::Help(String::from(HELPSTR)))
        } else { return Ok(make_config(tail.to_vec()))}
}