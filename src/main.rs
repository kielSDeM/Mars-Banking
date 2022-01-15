pub mod new_user;
mod settings;
pub mod user_account;
pub use crate::settings::settings;
use new_user::AccountCreation;
use new_user::CreateAccount;
use user_account::AccountInfo;
use user_account::User;

fn main() {
    loop {
        let mut select = String::new();
        println!("Welcome to Mars Banking!");
        println!("What would you like to do today?");
        println!("Create a new account: 1\nLogin: 2\nSettings: 3\nExit: 4");
        select.clear();
        std::io::stdin().read_line(&mut select);
        let select = select.trim();
        match select {
            "1" => AccountCreation::bank_new_user(),
            "2" => User::user_account(),
            "3" => settings(),
            "4" => break,
            _ => {}
        }
    }
}
