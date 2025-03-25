use crate::statement::StatementType::Unknown;

pub enum PrepareResult {
    Success,
    Unrecognized(String),
}

pub enum StatementType {
    Insert,
    Select,
    Unknown,
}

pub struct Statement {
    statement_type: StatementType,
}

impl Statement {
    pub fn new() -> Self {
        Self { statement_type: Unknown}
    }

    pub fn prepare(&mut self) -> PrepareResult {
        todo!()
    }

    pub fn execute(&self) -> Result<String, String> {
        todo!()
    }
}
