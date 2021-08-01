mod new_user;
pub use crate::new_user::bank_new_user;
use rand::Rng;

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
