use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum OwnerState {
    Owned,
    Borrowed,
    Moved,
    Frozen,
}

pub struct BorrowChecker {
    ownership: HashMap<String, OwnerState>,
    pub errors: Vec<String>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker {
            ownership: HashMap::new(),
            errors: Vec::new(),
        }
    }

    fn error(&mut self, msg: &str) {
        self.errors.push(format!("M0RX Memory Error: {}", msg));
    }

    pub fn declare(&mut self, name: &str) {
        self.ownership.insert(name.to_string(), OwnerState::Owned);
    }

    pub fn borrow(&mut self, name: &str) {
        match self.ownership.get(name) {
            Some(OwnerState::Moved) => {
                self.error(&format!(
                    "Cannot borrow '{}' — it has been moved", name
                ));
            }
            Some(OwnerState::Frozen) => {
                self.error(&format!(
                    "Cannot borrow '{}' — it is frozen", name
                ));
            }
            None => {
                self.error(&format!(
                    "'{}' is not declared", name
                ));
            }
            _ => {
                self.ownership.insert(
                    name.to_string(),
                    OwnerState::Borrowed,
                );
            }
        }
    }

    pub fn move_var(&mut self, name: &str) {
        match self.ownership.get(name) {
            Some(OwnerState::Moved) => {
                self.error(&format!(
                    "Cannot move '{}' — already moved", name
                ));
            }
            Some(OwnerState::Borrowed) => {
                self.error(&format!(
                    "Cannot move '{}' — currently borrowed", name
                ));
            }
            None => {
                self.error(&format!(
                    "'{}' is not declared", name
                ));
            }
            _ => {
                self.ownership.insert(
                    name.to_string(),
                    OwnerState::Moved,
                );
            }
        }
    }

    pub fn freeze(&mut self, name: &str) {
        self.ownership.insert(
            name.to_string(),
            OwnerState::Frozen,
        );
    }

    pub fn is_safe(&self, name: &str) -> bool {
        match self.ownership.get(name) {
            Some(OwnerState::Owned) |
            Some(OwnerState::Borrowed) => true,
            _ => false,
        }
    }
}
