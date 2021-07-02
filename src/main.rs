use rand::Rng;
fn bank_new_user() {
    let mut new_user = String::new();
    let mut account = String::new();
    let mut age = String::new();
    println!("Would you like to make a new account with us today?");
    loop {
        println!(" yes: continue to application, no: continue browsing , \
        or exit: to exit");
        std::io::stdin().read_line(&mut account)
            .ok()
            .expect("Failed to read line").to_string();
        let account = match account.trim().parse(){
            Ok(String) => String,
            Err(_) => continue,
        };
        if account == "yes" {
            println!("What is your name?");
            print!("Name: ");
            std::io::stdin().read_line(&mut new_user);
            println!(" ");
            println!("Hello {}, What is your age? ", new_user);
            std::io::stdin().read_line(&mut age);
            println!("We will generate a new account number \
            and routing number for.");
            let account_number =
                rand::thread_rng().gen_range(10000000..99999999);
            println!("Your account number is {}", account_number);
           let routing_number =
               rand::thread_rng().gen_range(10000000..99999999);
            println!("Your account routing number is {}", routing_number);
            break;
        } else if account == "no" {
            println!("You do not need an account to continue browsing.");
            println!("Have a wonderful day and thank you for considering Mars Banking!");
            break;
        } else if account == "exit" {
            println!("Thank you for choosing Mars Banking for your banking needs!\
                Have a wonderful day!");
            break;
        } else {
            println!("error please enter yes, no, or exit");
            continue;
        }
    }
    //TODO: Generate a random account number.

}

fn user_account(){
    println!("Welcome! How may I help you today?");
    /*TODO: We need to be able to deposit, withdraw, transfer,
       look at our transaction history and look at our account total.
       */
}

fn settings() {

}

fn main() {
    println!("Welcome to Mars Banking!");
    bank_new_user();
    user_account();
}
