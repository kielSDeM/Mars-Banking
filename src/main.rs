mod new_user;
mod user_account;

pub use crate::new_user::bank_new_user;

fn settings() {
    /*TODO: Just going to make 3 sets of settings, 1. turn card on or off
        2.Close Bank account, 3. Change username. I haven't implemented a function
        for changing the user name yet because I wanted to start on part2 of my
        project.
    */

}

fn main() {
    println!("Welcome to Mars Banking!");
    bank_new_user();
}
