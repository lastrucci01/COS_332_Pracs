pub struct DeleteRequest<'a> {
    base_dn: &'a str,
}

impl<'a> DeleteRequest<'a> {
    pub fn new(base_dn: &'a str) -> Self {
        Self { base_dn }
    }

    pub fn delete(&self, msg_id: u8, name: String) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x30, 0x2, 0x1, msg_id];

        let req = format!("cn={},{}", name, self.base_dn);
        let mut delete = vec![0x4a, req.len() as u8];
        delete.extend(req.as_bytes());

        bytes.extend(delete);
        bytes.insert(1, bytes.len() as u8 - 1);

        bytes
    }

    pub fn decode(&self, bytes: Vec<u8>) -> Option<()> {
        let pos = bytes
            .clone()
            .into_iter()
            .position(|byte| byte == 0x0a)
            .unwrap()
            + 2;

        let code = bytes[pos];

        match code {
            0 => Some(()),
            _ => None,
        }
    }
}
