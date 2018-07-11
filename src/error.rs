use core::convert::From;
use core::fmt;
use core::str::Utf8Error;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Invalid packet type in header
    PacketType,
    /// Invalid packet type flag in header
    PacketFlag,
    /// Malformed remaining length in header
    RemainingLength,
    /// Invalid buffer length
    InvalidLength,
    /// Invalid UTF-8 encoding
    Utf8,
    /// Invalid connect flag value
    InvalidConnectFlag,
    /// Invalid QoS value
    InvalidQoS,
    /// Invalid Will Retain value
    InvalidWillRetain,
    /// Cannot provide password without username
    PasswordWithoutUsername,
}

impl Error {
    fn desc(&self) -> &'static str {
        match *self {
            Error::PacketType => "invalid packet type in header",
            Error::PacketFlag => "invalid packet type flag in header",
            Error::RemainingLength => "malformed remaining length in header",
            Error::InvalidLength => "invalid buffer length",
            Error::Utf8 => "invalid utf-8 encoding",
            Error::InvalidConnectFlag => "invalid connect flag value",
            Error::InvalidQoS => "invalid qos value",
            Error::InvalidWillRetain => "invalid Will Retain value",
            Error::PasswordWithoutUsername => "cannot provide password without username",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.desc())
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        self.desc()
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::Utf8
    }
}
