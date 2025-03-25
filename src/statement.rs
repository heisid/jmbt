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
        Self { statement_type: StatementType::Unknown}
    }

    pub fn prepare(&mut self) -> PrepareResult {
        todo!()
    }

    pub fn execute(&self) -> Result<String, String> {
        todo!()
    }
}
