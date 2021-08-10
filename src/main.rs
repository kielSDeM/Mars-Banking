mod ll;
mod new_user;
mod settings;
mod user_account;

pub use crate::new_user::bank_new_user;
pub use crate::settings::settings;
pub use crate::user_account::user_account;

fn main() {
    let age3 = String::new();

    let mut select = String::new();
    println!("Welcome to Mars Banking!");
    loop {
        println!("What would you like to do today?");
        println!("Create a new account: 1\nLogin: 2\nSettings: 3\nExit: 4");
        select.clear();
        std::io::stdin().read_line(&mut select);
        let select = select.trim();
        match select {
            "1" => bank_new_user(),
            "2" => user_account(&age3),
            "3" => settings(),
            "4" => break,
            _ => {}
        }
    }
}
