use super::{cell_with_read::CellWithRead, zone::Zone};

pub struct RefZone<'a> {
    pub zone: &'a Zone,
    pub cells: Vec<CellWithRead<'a>>,
}
