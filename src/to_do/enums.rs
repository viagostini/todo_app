use std::fmt;

pub enum TaskStatus {
    Done,
    Pending,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Done => write!(f, "Done"),
            Self::Pending => write!(f, "Pending"),
        }
    }
}
