pub type Result<T> = std::result::Result<T, WngError>;

#[macro_export]
macro_rules! error {
    () => {
        WngError {
            line: line!(),
            file: file!().to_string(),
            message: "".to_owned(),
        }
    };
    ($($msg:tt),*) => {
        {
            let mut message = String::new();

            $(
                message.push_str(&format!("{} ", $msg));
            )*

            message.pop(); // the trailing whitespace

            WngError {
                line: line!(),
                file: file!().to_string(),
                message,
            }
        }
    };
}

impl From<std::io::Error> for WngError {
    fn from(error: std::io::Error) -> Self {
        let msg = format!("{}", error);
        error!(msg)
    }
}

impl From<toml::de::Error> for WngError {
    fn from(error: toml::de::Error) -> Self {
        let msg = format!("{}", error);
        error!(msg)
    }
}

#[derive(PartialEq, Eq)]
pub struct WngError {
    pub line: u32,
    pub file: String,
    pub message: String,
}

impl std::fmt::Display for WngError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}:{}", self.message, self.file, self.line)
    }
}


impl std::fmt::Debug for WngError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}:{}", self.message, self.file, self.line)
    }
}
