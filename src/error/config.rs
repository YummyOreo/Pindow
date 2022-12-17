use std::fmt;

#[derive(Debug, Clone)]
pub struct SetConfigError {
    pub num: i32,
}

impl fmt::Display for SetConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to set the config to: {}. Try checking if that config exists!",
            self.num
        )
    }
}

#[derive(Debug, Clone)]
pub enum LoadConfigError {
    StringToKeycodeError(String),
    StringToEventError(String),
    InvalidNumber(String),
}

impl fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::StringToKeycodeError(s) => write!(f, "Could not convert \"{}\" to a keycode. \n\t- Try looking at documentation for a list of kecode (it might not be supported) \n\t- The events are case-sensitive", s),
            &Self::StringToEventError(s) => write!(f, "Could not convert \"{}\" to a event. \n\t- Try looking at documentation for a list of events \n\t- The events are case-sensitive", s),
            &Self::InvalidNumber(s) => write!(f, "Number \"{}\" is invalid, there are multiple possibilities: \n\t- Number is greater than 9 \n\t- Number is 0 or less \n\t- Number is not a number \nPlease resolve these or look at documentation for help.", s)
        }
    }
}
