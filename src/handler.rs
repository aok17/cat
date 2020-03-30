
pub enum Error { 
    NoProg,
    NoFile(String),
    NoPerm(String),
    Other(String)
}

const TERMINAL: &str = "powershell";
const PROG: &str = "type";

pub fn is_no_file_error(e:String) -> bool {
    e.contains("file")
}

pub fn is_permission_denied_error(e:String) -> bool {
    e.contains("permission")
}

pub fn exec(cmd:String,args:Vec<String>) -> Result<String,Error>{
    let output = std::process::Command::new(TERMINAL).arg(cmd).args(args).output();
    match output {
        Ok(out) => { 
            if !out.status.success() {
                return Err(Error::Other(String::from("Program exited with ")+&out.status.to_string()))
            } else{
                return Ok(String::from_utf8(out.stdout).unwrap())
            }

        },
        Err(e) => {
            if is_no_file_error(e.to_string()){return Err(Error::NoFile(e.to_string()))}
            if is_permission_denied_error(e.to_string()){return Err(Error::NoPerm(e.to_string()))}
            else {return Err( Error::Other(e.to_string()))}
        }
    }
}

pub fn run(filepaths:Vec<String>) -> String {
    
    let result = exec(PROG.to_string(),filepaths);
    
    match result {
        Ok(msg) => return msg,
        Err(e) => match e {
            Error::NoProg => return String::from("No '")+PROG+"' program found",
            Error::NoFile(msg) => return msg,
            Error::NoPerm(msg) => return msg,
            Error::Other(msg) => return msg
        }
    }
}