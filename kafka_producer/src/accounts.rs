use serde_json::json;

pub struct Accounts {
  accounts: Vec<String>
}

impl Accounts {

  pub fn new() -> Self {
    Self {
      accounts: vec![]
    }
  }

  pub fn add_account(&mut self, account: String) {
    self.accounts.push(account);
  }

  pub fn remove_account(&mut self, index: usize) {
    self.accounts.remove( index );
  }

  pub fn to_json(&self) -> String {
    json!(&self.accounts).to_string()
  }
}
