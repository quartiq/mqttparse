/// The result of a successful parse pass. Taken from the `httparse` crate.
///
/// `Complete` is used when the buffer contained the complete value.
/// `Partial` is used when parsing did not reach the end of the expected value,
/// but no invalid data was found.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status<T> {
    /// The completed result.
    Complete(T),
    /// A partial result.
    Partial,
}

impl<T> Status<T> {
    /// Convenience method to check if status is complete.
    #[inline]
    pub fn is_complete(&self) -> bool {
        match *self {
            Status::Complete(..) => true,
            Status::Partial => false,
        }
    }

    /// Convenience method to check if status is partial.
    #[inline]
    pub fn is_partial(&self) -> bool {
        match *self {
            Status::Complete(..) => false,
            Status::Partial => true,
        }
    }

    /// Convenience method to unwrap a Complete value. Panics if the status is
    /// `Partial`.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Status::Complete(t) => t,
            Status::Partial => panic!("Tried to unwrap Status::Partial"),
        }
    }
}

#[macro_export]
macro_rules! complete {
    ($e:expr) => {
        match $e? {
            Status::Complete(v) => v,
            Status::Partial => return Ok(Status::Partial),
        }
    };
}

#[macro_export]
macro_rules! read_byte {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let b = $bytes[$read];
            $read += 1;
            b
        } else {
            return Ok(Status::Partial);
        }
    }};
}

#[macro_export]
macro_rules! read_bytes {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let s = complete!(decode_len_prefixed_bytes(&$bytes[$read..]));
            $read += 2 + s.len();
            s
        } else {
            return Ok(Status::Partial);
        }
    }};
}

#[macro_export]
macro_rules! read_bytes_final {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let s = complete!(decode_len_prefixed_bytes(&$bytes[$read..]));
            s
        } else {
            return Ok(Status::Partial);
        }
    }};
}

#[macro_export]
macro_rules! read_str {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let s = complete!(decode_string(&$bytes[$read..]));
            $read += 2 + s.len();
            s
        } else {
            return Ok(Status::Partial);
        }
    }};
}

#[macro_export]
macro_rules! read_str_final {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let s = complete!(decode_string(&$bytes[$read..]));
            s
        } else {
            return Ok(Status::Partial);
        }
    }};
}

#[macro_export]
macro_rules! read_u16 {
    ($bytes:ident, $read:ident) => {{
        if $bytes.len() - $read > 0 {
            let v = BigEndian::read_u16(&$bytes[$read..]);
            $read += 2;
            v
        } else {
            return Ok(Status::Partial);
        }
    }};
}
