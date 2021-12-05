use rand::Rng;
//just asks for the users name and age and no more personal info for incase someone wants to check
//out this repo.
#[allow(dead_code)]

pub struct NewUser {
    age: String,
    new_user: String,
    account_number: i32,
    routing_number: i32,
}

pub trait UserInfo {
    fn user_info();
}

pub trait CreateAccount {
    fn bank_new_user();
}

pub struct AccountCreation {
    account: String,
}
//function for user info.
impl UserInfo for NewUser {
    fn user_info() {
        let mut user_info = NewUser {
            age: String::new(),
            new_user: String::new(),
            account_number: rand::thread_rng().gen_range(10000000..99999999),
            routing_number: rand::thread_rng().gen_range(10000000..99999999),
        };
        println!("What is your name?");
        print!("Name: ");
        std::io::stdin().read_line(&mut user_info.new_user);
        println!(" ");
        println!("Hello {}, What is your age? ", user_info.new_user);
        std::io::stdin().read_line(&mut user_info.age);
        println!(
            "We will generate a new account number \
               and routing number for you."
        );
        println!("Your account number is {}", user_info.account_number);
        println!(
            "Your account routing number is {}",
            user_info.routing_number
        );
    }
}
fn yes() {
    NewUser::user_info();
}
impl CreateAccount for AccountCreation {
    fn bank_new_user() {
        let mut new_user: AccountCreation = AccountCreation {
            account: String::new(),
        };
        println!("Would you like to make a new account with us today?");
        loop {
            println!(
                " yes: continue to application, no: continue browsing , \
            or exit: to exit"
            );
            new_user.account.clear();
            std::io::stdin()
                .read_line(&mut new_user.account)
                .expect("please type yes, no or exit.");
            let account = new_user.account.trim();
            match account {
                "yes" => {
                    yes();
                    continue;
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
