pub use crate::user_account::user_account;
use rand::Rng;
//just asks for the users name and age and no more personal info for incase someone wants to check
//out this repo.

//function for user info.
pub fn user_info() {
    let mut age = String::new();
    let mut new_user = String::new();
    println!("What is your name?");
    print!("Name: ");
    std::io::stdin().read_line(&mut new_user);
    println!(" ");
    println!("Hello {}, What is your age? ", new_user);
    std::io::stdin().read_line(&mut age);
    let age2: String = age.trim().into();
    user_account(&age2)
}
fn acc_no() {
    println!(
        "We will generate a new account number \
               and routing number for you."
    );
    let account_number = rand::thread_rng().gen_range(10000000..99999999);
    println!("Your account number is {}", account_number);
    let routing_number = rand::thread_rng().gen_range(10000000..99999999);
    println!("Your account routing number is {}", routing_number);
}
//function for the yes input in the bank_new_user function.
fn yes() {
    user_info();
    acc_no();
}
//function that takes input for making a new account.
pub fn bank_new_user() {
    let mut account = String::new();
    println!("Would you like to make a new account with us today?");
    loop {
        println!(
            " yes: continue to application, no: continue browsing , \
        or exit: to exit"
        );
        account.clear();
        std::io::stdin()
            .read_line(&mut account)
            .expect("please type yes, no or exit.");
        let account = account.trim();
        match account {
            "yes" => {
                yes();
                break;
            }

            "no" => {
                println!("You do not need an account to continue browsing.");
                println!("Have a wonderful day and thank you for considering Mars Banking!");
                break;
            }
            "exit" => {
                println!(
                    "Thank you for choosing Mars Banking for your banking needs!\
                Have a wonderful day!"
                );
                break;
            }
            _ => {
                println!("Error! Enter yes, no, or exit.")
            }
        }
    }
}
