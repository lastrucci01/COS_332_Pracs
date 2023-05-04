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
            0x30, // sequence
            // message id
            0x02, 0x01, msg_id, // TLV triplet
        ];

        let mut bind = vec![
            0x60, // bind type
            0x02, 0x01, 0x03, //versin
        ];

        // name
        let name_len = self.name.len();

        bind.push(0x04);
        bind.push(name_len as u8);
        bind.extend_from_slice(self.name.as_bytes());

        // auth
        let auth_len = self.auth.len();

        bind.push(0x80);
        bind.push(auth_len as u8);
        bind.extend_from_slice(self.auth.as_bytes());

        //
        let seq_len = bind.len();

        bind.insert(1, seq_len as u8 - 1);

        bytes.extend(bind);
        bytes.insert(1, bytes.len() as u8 - 1);
        bytes
    }

    pub fn unbind(&self, msg_id: u8) -> Vec<u8> {
        vec![0x30, 0x05, 0x02, 0x01, msg_id, 0x42, 0x00]
    }
}
