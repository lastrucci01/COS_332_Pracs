pub struct BindRequest<'a> {
    name: &'a str,
    auth: &'a str,
}

impl<'a> BindRequest<'a> {
    pub fn new(name: &'a str, auth: &'a str) -> Self {
        Self { name, auth }
    }

    pub fn bind(&self, msg_id: u8) -> Vec<u8> {
        // Encode the version field
        let mut bytes = vec![
            0x30,  // sequence
            // version
            0x02,
            0x01,
            0x03, // TLV triplet
            // type
            0x02,
            0x01,
            0x00,
            // message_id
            0x02,
            0x01,
            msg_id,
        ];

        // name
        let name_len = self.name.len();

        bytes.push(0x04);
        bytes.push(name_len as u8);
        bytes.extend_from_slice(self.name.as_bytes());

        // auth
        let auth_len = self.auth.len();
        
        bytes.push(0x80);
        bytes.push(auth_len as u8);
        bytes.extend_from_slice(self.auth.as_bytes());

        // 
        let seq_len = bytes.len();
        bytes.insert(0, 0x60);
        bytes.insert(1, seq_len as u8);
        
        bytes
    }
}
