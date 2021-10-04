pub mod new_user;
mod settings;
mod user_account;
pub use crate::settings::settings;
pub use crate::user_account::user_account;
use new_user::NewUser;
use new_user::UserInfo;
fn main() {
    loop{
        let mut select = String::new();
            println!("Welcome to Mars Banking!");
            println!("What would you like to do today?");
            println!("Create a new account: 1\nLogin: 2\nSettings: 3\nExit: 4");
            select.clear();
            std::io::stdin().read_line(&mut select);
            let select = select.trim();
            match select {
                "1" => NewUser::bank_new_user(self),
                "2" => user_account(),
                "3" => settings(),
                "4" => break,
                _ => {}
            }
        }
    }

