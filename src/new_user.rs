pub use crate::user_account::user_account;
use rand::Rng;
//just asks for the users name and age and no more personal info for incase someone wants to check
//out this repo.
#[allow(dead_code)]
pub trait UserInfo {
    fn user_info(&mut self);
    fn acc_no(&mut self);
    fn yes(self);
    fn bank_new_user(self);
}
pub struct NewUser {
    age: String,
    new_user: String,
    account: String,
    account_number: i32,
    routing_number: i32,
    select: String,
}
//function for user info.
impl UserInfo for NewUser {
    fn user_info(&mut self) {
        self.age = String::new();
        self.new_user = String::new();
        println!("What is your name?");
        print!("Name: ");
        std::io::stdin().read_line(&mut self.new_user);
        println!(" ");
        println!("Hello {}, What is your age? ", self.new_user);
        std::io::stdin().read_line(&mut self.age);
        let age2: String = self.age.trim().into();

    }

    fn acc_no(&mut self) {
        println!(
            "We will generate a new account number \
               and routing number for you."
        );
        self.account_number = rand::thread_rng().gen_range(10000000..99999999);
        println!("Your account number is {}", self.account_number);
        self.routing_number = rand::thread_rng().gen_range(10000000..99999999);
        println!("Your account routing number is {}", self.routing_number);
    }
    //function for the yes input in the bank_new_user function.
    fn yes(self) {
        NewUser::user_info(&mut self);
        NewUser::acc_no(&mut self);
    }
    //function that takes input for making a new account.
    fn bank_new_user(self) {
        self.account = String::new();
        println!("Would you like to make a new account with us today?");
        loop {
            println!(
                " yes: continue to application, no: continue browsing , \
        or exit: to exit"
            );
            self.account.clear();
            std::io::stdin()
                .read_line(&mut self.account)
                .expect("please type yes, no or exit.");
            let account = self.account.trim();
            match account {
                "yes" => {
                    self.yes();
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
}
