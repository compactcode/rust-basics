use accord::{Accord, Result as AccordResult, MultipleError, MultipleInvalid};
use accord::validators::{min};

#[derive(FromForm)]
pub struct Task {
    pub description: String,
    pub done: bool
}

impl Accord for Task {
    fn validate(&self) -> AccordResult {
        rules!{
            "description" => self.description => [min(1)]
        }
    }
}
