use super::{parse_len_prefixed_bytes, parse_string, Error, QoS, Result, Status};
use byteorder::{BigEndian, ByteOrder};
use core::time::Duration;

#[derive(Debug, PartialEq)]
pub struct Connect<'buf> {
    name: &'buf str,
    revision: u8,
    flags: u8,
    clean_session: bool,
    will_flag: bool,
    will_topic: Option<&'buf str>,
    will_msg: Option<&'buf [u8]>,
    will_qos: QoS,
    will_retain: bool,
    username_present: bool,
    username: Option<&'buf str>,
    password_present: bool,
    password: Option<&'buf [u8]>,
    keep_alive: Duration,
    client_id: &'buf str,
}

impl<'buf> Connect<'buf> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Status<Connect>> {
        // read protocol name
        let mut read = 0;
        let name = read_str!(bytes, read);

        // read protocol revision
        let revision = read_byte!(bytes, read);

        // read protocol flags
        let flags = read_byte!(bytes, read);

        // MQTT-3.1.2-3 requires that the LSB be always set to 0
        if flags & 1 != 0 {
            return Err(Error::InvalidConnectFlag);
        }

        let clean_session = flags & 0b000_000_10 == 1;
        let will_flag = flags & 0b000_001_00 == 1;
        let will_qos = QoS::from_u8(flags & 0b000_110_00)?;
        let will_retain = flags & 0b001_000_00 == 1;
        let password_present = flags & 0b010_000_00 == 1;
        let username_present = flags & 0b100_000_00 == 1;

        // MQTT-3.1.2-11 - If the Will Flag is set to 0 the Will QoS and Will
        // Retain fields in the Connect Flags MUST be set to zero and the Will
        // Topic and Will Message fields MUST NOT be present in the payload
        if !will_flag {
            if will_qos != QoS::AtMostOnce {
                return Err(Error::InvalidQoS);
            }
            if will_retain {
                return Err(Error::InvalidWillRetain);
            }
        }

        // MQTT-3.1.2-22 - If the User Name Flag is set to 0, the Password Flag MUST be set to 0
        if !username_present && password_present {
            return Err(Error::PasswordWithoutUsername);
        }

        // read keep alive duration
        let keep_alive = Duration::from_secs(read_u16!(bytes, read) as u64);

        let client_id = read_str!(bytes, read);

        // read will topic name & message
        let mut will_topic = None;
        let mut will_msg = None;
        if will_flag {
            will_topic = Some(read_str!(bytes, read));
            will_msg = Some(read_bytes!(bytes, read));
        }

        // read user name
        let mut username = None;
        if username_present {
            username = Some(read_str!(bytes, read));
        }

        // read user name
        let mut password = None;
        if password_present {
            password = Some(read_bytes_final!(bytes, read));
        }

        Ok(Status::Complete(Connect {
            name,
            revision,
            flags,
            clean_session,
            will_flag,
            will_qos,
            will_retain,
            username_present,
            username,
            password_present,
            password,
            keep_alive,
            client_id,
            will_topic,
            will_msg,
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

    pub fn clean_session(&self) -> &bool {
        &self.clean_session
    }

    pub fn will_flag(&self) -> &bool {
        &self.will_flag
    }

    pub fn will_qos(&self) -> &QoS {
        &self.will_qos
    }

    pub fn will_retain(&self) -> &bool {
        &self.will_retain
    }

    pub fn username_present(&self) -> &bool {
        &self.username_present
    }

    pub fn username(&self) -> &Option<&'buf str> {
        &self.username
    }

    pub fn password_present(&self) -> &bool {
        &self.password_present
    }

    pub fn password(&self) -> &Option<&'buf [u8]> {
        &self.password
    }

    pub fn keep_alive(&self) -> &Duration {
        &self.keep_alive
    }

    pub fn client_id(&self) -> &'buf str {
        self.client_id
    }

    pub fn will_topic(&self) -> &Option<&'buf str> {
        &self.will_topic
    }

    pub fn will_msg(&self) -> &Option<&'buf [u8]> {
        &self.will_msg
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
