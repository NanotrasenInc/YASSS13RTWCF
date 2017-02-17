use super::ID;

#[derive(Hash, Debug)]
pub struct Entity {
    id: ID
}

impl Entity {
    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl PartialEq<u64> for Entity {
    fn eq(&self, other: &u64) -> bool {
        self.id == *other
    }
}
