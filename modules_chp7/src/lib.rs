mod internet_banking;

pub use crate::internet_banking::user_acc;

pub fn user_login(){
    user_acc::account_balance();
    user_acc::funds_transfer();
    user_acc::view_statements();
    user_acc::update_password();
}

pub mod credit_card;
// load content of credit_card module