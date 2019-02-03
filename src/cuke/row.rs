use cuke::Cell;

#[derive(Debug, Clone)]
pub struct Row<'d> {
    pub cells: Vec<Cell<'d>>,
}
