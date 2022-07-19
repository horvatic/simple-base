pub struct Packet {
    data: Option<Vec<u8>>
}

pub fn new_packet(data: Option<Vec<u8>>) -> Packet {
    return Packet { data }
}

impl Packet {
    pub fn get_data(&self) -> Option<Vec<u8>> {
        return self.data.clone();
    }
}