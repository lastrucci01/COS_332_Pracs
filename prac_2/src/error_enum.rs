pub enum ErrorChecksEnum {
    AddContactCheck {commands: Vec<&'static str> }
}

impl ErrorChecksEnum {
    pub fn check(&self) -> Result<(), &str> {
        match &*self {
            ErrorChecksEnum::AddContactCheck { commands } => {
                if commands.len() != 3 {
                } 
                Ok(())
            },
        }
    }
}