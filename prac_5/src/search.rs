use std::{vec, collections::HashMap};

pub struct SearchRequest<'a> {
    base_dn: &'a str,
    attribute: &'a str,
}

impl<'a> SearchRequest<'a> {
    pub fn new(base_dn: &'a str) -> Self {
        Self {
            base_dn,
            attribute: "cn",
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

    pub fn decode(bytes: Vec<u8>) -> HashMap<String, String> {
        // println!("{:x?}", bytes);
        let mut index_of_64: usize;
        let mut count: usize = 0;
        let mut results: Vec<String> = Vec::new();

        while count < bytes.len() {
            let byte_val = bytes[count];
            if byte_val == 0x64 {
                index_of_64 = count;
                let entry_len = bytes[index_of_64 + 1];
                let mut curr_pos = index_of_64 + 2;

                while curr_pos < bytes.len() && curr_pos < (index_of_64 + entry_len as usize) {
                    if bytes[curr_pos] == 0x04 {
                        let attr_len = bytes[curr_pos + 1];

                        let from = curr_pos + 2;
                        let to = from + attr_len as usize;
                        results.push(String::from_utf8(bytes[from..to].to_vec()).unwrap());
                        curr_pos = to.clone();
                        continue;
                    }
                    curr_pos += 1
                }
                count = curr_pos;
            } else {
                count += 1;
            }
        }

        let mut map: HashMap<String, String> = HashMap::new();
        let mut i = 0;
        while i + 1 < results.len() {
            if results[i] == "cn" {
                let key = String::from("name");
                let val = results[i + 1].to_owned();
                map.insert(key, val);
            } else if results[i] == "telephoneNumber" {
                let key = String::from("speed");
                let val = results[i + 1].to_owned();
                map.insert(key, val);
            }

            i += 1;
        }

        map
    }
}
