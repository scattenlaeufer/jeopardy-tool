use std::{
    fmt,
    process::{ExitCode, Termination},
};

pub enum ToolResult<T> {
    Ok(T),
    Err(ToolError),
}

impl<T> Termination for ToolResult<T> {
    fn report(self) -> ExitCode {
        match self {
            Self::Ok(_) => ExitCode::SUCCESS,
            Self::Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}

#[derive(Debug)]
pub enum ToolError {
    Other(String),
}

impl std::error::Error for ToolError {}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Other(e) => write!(f, "Some other error: {}", e),
        }
    }
}

pub fn show(prefix: Option<String>) -> ToolResult<()> {
    println!("{:?}", prefix);
    ToolResult::Err(ToolError::Other("Some error".to_string()))
}
