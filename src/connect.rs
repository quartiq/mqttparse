use super::{parse_string, Result, Status};

#[derive(Debug, PartialEq)]
pub struct Connect<'buf> {
    name: &'buf str,
    revision: u8,
    flags: u8,
}

impl<'buf> Connect<'buf> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Status<Connect>> {
        // read protocol name
        let name = complete!(parse_string(bytes));
        let mut read = 2 + name.len(); // 2 bytes for the string len prefix + length of string in bytes

        // read protocol revision
        let revision = next!(bytes, read);
        read += 1;

        // read protocol flags
        let flags = next!(bytes, read);

        Ok(Status::Complete(Connect {
            name,
            revision,
            flags,
        }))
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn revision(&self) -> &u8 {
        &self.revision
    }

    pub fn flags(&self) -> &u8 {
        &self.flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, WriteBytesExt};
    use std::io::{Cursor, Write};

    fn encode_str(s: &str) -> Cursor<Vec<u8>> {
        let mut buf = Cursor::new(Vec::new());
        buf.write_u16::<BigEndian>(s.len() as u16).unwrap();
        buf.write(s.as_bytes()).unwrap();

        buf
    }

    #[test]
    fn insufficient_buf() {
        assert_eq!(Status::Partial, Connect::from_bytes(&[]).unwrap());
        assert_eq!(Status::Partial, Connect::from_bytes(&[1]).unwrap());
        assert_eq!(
            Status::Partial,
            Connect::from_bytes(encode_str("MQTT").get_ref().as_ref()).unwrap()
        );

        let mut buf = encode_str("MQTT");
        buf.write(&[0]).unwrap();
        assert_eq!(
            Status::Partial,
            Connect::from_bytes(buf.get_ref().as_ref()).unwrap()
        );
    }

    #[test]
    fn parse_connect() {
        let mut buf = encode_str("MQTT");
        buf.write(&[1, 2]).unwrap(); // protocol revision + protocol flags
        let conn = Connect::from_bytes(buf.get_ref().as_ref())
            .unwrap()
            .unwrap();
        assert_eq!(conn.name(), "MQTT");
        assert_eq!(*conn.revision(), 1);
        assert_eq!(*conn.flags(), 2);
    }
}
