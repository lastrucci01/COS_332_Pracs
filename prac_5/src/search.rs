use std::vec;

pub struct SearchRequest<'a> {
    base_dn: &'a str,
    attribute: &'a str,
}
impl<'a> SearchRequest<'a> {
    pub fn new(base_dn: &'a str) -> Self {
        Self {
            base_dn,
            attribute: "uidNumber",
        }
    }

    pub fn search(&self, msg_id: u8, filter: String) -> Vec<u8> {
        let mut bytes = vec![0x30, 0x02, 0x01, msg_id];
        let mut search: Vec<u8> = vec![0x63];

        // dn
        search.push(0x04);
        search.push(self.base_dn.len() as u8);
        search.extend_from_slice(self.base_dn.as_bytes());

        // scope
        search.extend(vec![0x0a, 0x01, 0x02]);
        // deref alias
        search.extend(vec![0x0a, 0x01, 0x00]);
        // size
        search.extend(vec![0x02, 0x01, 0x00]);
        // time
        search.extend(vec![0x02, 0x01, 0x00]);
        // types
        search.extend(vec![0x01, 0x01, 0x00]);

        // filter
        let mut choice: Vec<u8> = vec![0xa3];
        choice.extend(vec![0x04, self.attribute.len() as u8]);
        choice.extend_from_slice(self.attribute.as_bytes());
        //attribute
        choice.extend(vec![0x04, filter.len() as u8]);
        choice.extend_from_slice(filter.as_bytes());

        choice.insert(1, choice.len() as u8 - 1);

        search.extend(choice);

        search.append(&mut vec![0x30, 0x00]);

        search.insert(1, search.len() as u8 - 1);

        bytes.extend(search);
        bytes.insert(1, bytes.len() as u8 - 1);

        bytes
    }
}
