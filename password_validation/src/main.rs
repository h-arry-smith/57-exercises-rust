use std::io;
use std::io::Write;

struct Credential {
    username: String,
    password: String,
}

impl Credential {
    fn authenticate(&self, password: &str) -> bool {
        self.password == password
    }
}

struct AccountStore {
    accounts: Vec<Credential>,
}

impl AccountStore {
    fn new() -> Self {
        AccountStore {
            accounts: Vec::new(),
        }
    }

    fn create_account(&mut self, username: String, password: String) {
        let credential = Credential { username, password };

        self.accounts.push(credential);
    }

    fn find_account(&self, username: &str) -> Option<&Credential> {
        self.accounts
            .iter()
            .find(|&account| account.username == username)
    }
}

fn main() {
    let mut account_store = AccountStore::new();

    account_store.create_account("harry".to_string(), "test".to_string());
    account_store.create_account("dave".to_string(), "password".to_string());

    let username = get_string("Username: ");
    let password = get_string("Password: ");

    let account = account_store.find_account(&username);

    match account {
        Some(account) => {
            if account.authenticate(&password) {
                println!("Log in Successful!");
            } else {
                println!("Invalid username/password combination!");
            }
        }
        None => {
            println!("Invalid username/password combination!");
        }
    }
}

fn get_string(question_text: &str) -> String {
    let mut input = String::new();

    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a string.");

        if input != "" {
            return input.to_string();
        }
    }
}
