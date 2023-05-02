use std::vec;

pub struct SearchRequest<'a> {
    base_dn: &'a str,
    attribute: &'a str,
}
impl<'a> SearchRequest<'a> {
    pub fn new(base_dn: &'a str) -> Self {
        Self {
            base_dn,
            attribute: "telephone",
        }
    }

    pub fn search(&self, msg_id: u8, filter: String) -> Vec<u8> {
        let mut bytes = vec![0x30, 0x0, 0x02, 0x01, msg_id, 0x63, 0x51];

        // dn
        bytes.push(0x04);
        bytes.push(self.base_dn.len() as u8);
        bytes.extend_from_slice(self.base_dn.as_bytes());

        // scope
        bytes.extend(vec![0x0a, 0x01, 0x02]);
        // deref alias
        bytes.extend(vec![0x0a, 0x01, 0x03]);
        // size
        bytes.extend(vec![0x02, 0x01, 0x00]);
        // types
        bytes.extend(vec![0x02, 0x01, 0x00]);

        // filter
        bytes.extend(vec![0x04, filter.len() as u8]);
        bytes.extend_from_slice(filter.as_bytes());

        //attribute
        bytes.extend(vec![0x04, self.attribute.len() as u8]);
        bytes.extend_from_slice(self.attribute.as_bytes());

        
        bytes.append(&mut vec![0x30, 0x06, 0x04, 0x01, 0x2a, 0x04, 0x01, 0x2b]);
        
        bytes[1] = (bytes.len() - 2) as u8;
        println!("{:x?}", bytes.clone());
        bytes
    }
}
