use std::cell::RefCell;
use std::fmt;
use std::ops::{self, Not};

#[derive(Debug, Clone)]
pub(crate) enum Status {
    Idle,
    /// output buffer full
    OBF,
    ///input buffer full
    IBF,
    Not(Box<Status>),
}

impl Status {
    #[inline]
    pub fn satisfied(&self, status: u8) -> bool {
        match self {
            Status::Idle => (status & 0b11) == 0b00,
            Status::OBF => (status & 0b01) == 0b01,
            Status::IBF => (status & 0b10) == 0b10,
            Status::Not(v) => !(*v).satisfied(status),
        }
    }
}

impl ops::Not for Status {
    type Output = Status;
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Status::Not(v) => *v,
            _ => Status::Not(Box::new(self)),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Idle => write!(f, "IDLE"),
            Status::OBF => write!(f, "OBF=1"),
            Status::IBF => write!(f, "IBF=1"),
            Status::Not(v) => {
                let mut count = 1;
                let mut temp: &Status = &*v;
                while let Status::Not(v) = temp {
                    count = count + 1;
                    temp = &*v;
                }
                if count % 2 == 0 {
                    return temp.fmt(f);
                }
                match temp {
                    Status::Idle => write!(f, "IBF=1 or OBF=1"),
                    Status::OBF => write!(f, "OBF=0"),
                    Status::IBF => write!(f, "IBF=0"),
                    _ => write!(f, ""), // impossible here
                }
            }
        }
    }
}
