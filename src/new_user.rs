use rand::Rng;
use std::io::Read;


fn yes(){
    let mut new_user = String::new();
    let mut age = String::new();
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
}

pub fn bank_new_user() {
    let mut account= String::new();
    println!("Would you like to make a new account with us today?");
    loop {
        println!(" yes: continue to application, no: continue browsing , \
        or exit: to exit");
        std::io::stdin().read_line(&mut account);
        let account = account.trim();
       match account {
           "yes" => yes(),

           "no" => {
               println!("You do not need an account to continue browsing.");
               println!("Have a wonderful day and thank you for considering Mars Banking!");
               break;
       }
           "exit" => {
               println!("Thank you for choosing Mars Banking for your banking needs!\
                Have a wonderful day!");
               break;
        }
            _ => {println!("Error! Enter yes, no, or exit.")}
        }
    }
}

