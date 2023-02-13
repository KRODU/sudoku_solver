use super::zone::Zone;

pub struct ZoneKeyMap<T> {
    pub arr: Box<[(Zone, Option<T>)]>,
}

impl<T> ZoneKeyMap<T> {
    // pub fn new(max_index: usize) -> Self {

    // }
}
