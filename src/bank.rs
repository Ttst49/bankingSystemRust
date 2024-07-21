use crate::show_menu;
use crate::user::User;
use postgres::Client;
use rand::Rng;
use std::io::stdin;

pub struct BankAccount {
    pub account_id: i32,
    pub user_id: i32,
    pub account_type: String,
    pub balance: f32,
    pub interest_rate: f32,
    pub owner: String,
}

impl BankAccount {
    pub fn new(
        user: &mut Option<User>,
        client: &mut Client,
        account_type: String,
        mut balance: f32,
    ) -> BankAccount {
        let new_user = user.as_mut().expect("User is required");
        let first_name = new_user.first_name.as_ref().expect("First name is required").as_str();
        let last_name = new_user.last_name.as_ref().expect("Last name is required").as_str();
        let full_name = format!("{} {}", first_name, last_name);

        if balance.is_nan() {
            balance = 0.0
        }
        let mut rng = rand::thread_rng();
        let interest_rate = rng.gen_range(0.0..6.0);
        let bank_account = BankAccount {
            account_id: 0,
            user_id: user.as_mut().unwrap().id,
            account_type,
            balance,
            interest_rate,
            owner: full_name,
        };

        if bank_account.user_id.is_positive() {
            client
                .execute(
                    "INSERT INTO bank_account\
                 (account_id,user_id,account_type,balance,interest_rate,owner) \
                 VALUES ($1,$2,$3,$4,$5,$6)",
                    &[
                        &bank_account.account_id,
                        &bank_account.user_id,
                        &bank_account.account_type,
                        &bank_account.balance,
                        &bank_account.interest_rate,
                        &bank_account.owner,
                    ],
                )
                .expect("Everything panicked");
        }
        bank_account
    }

    pub fn create_bank_account(user: &mut Option<User>, client: &mut Client) {
        println!(
            "Hey {},pour l'ouverture de votre compte vous devez préciser le montant à placer ainsi que le type de compte:",
            user.as_ref().unwrap().username
        );
        let mut balance = String::from("0.0");
        let mut account_type = String::new();
        println!("Quel montant voulez-vous placez sur votre compte?");
        stdin().read_line(&mut balance).unwrap();
        println!("Quel est le type du compte ? (épargne, livret A,...)");
        stdin().read_line(&mut account_type).unwrap();
        BankAccount::new(user, client, account_type, balance.parse::<f32>().unwrap());
        show_menu(user, client)
    }
}
