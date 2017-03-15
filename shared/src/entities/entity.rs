use super::ID;

#[derive(Hash, Debug)]
pub struct Entity {
    id: ID,
}

impl Entity {
    pub fn get_id(&self) -> ID {
        self.id
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl PartialEq<ID> for Entity {
    fn eq(&self, other: &ID) -> bool {
        self.id == *other
    }
}
