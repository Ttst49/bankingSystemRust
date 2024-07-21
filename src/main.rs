mod bank;
mod user;

use crate::bank::BankAccount;
use crate::user::User;
use postgres;
use postgres::{Client, NoTls};
use std::io::stdin;

fn show_menu(user: &mut Option<User>, client: &mut Client) {
    if user.is_none() {
        println!(
            "\n
        --------------------\n
        | 1:Login          |\n
        | 2:Register       |\n
        --------------------\n
     "
        )
    } else {
        println!(
            "\n
        ---------------------------\n
        | 1:Créer compte bancaire |\n
        | 2:Remove a todo         |\n
        | 3:Edit a todo           |\n
        | 4:Show todolist         |\n
        | 5:Reset todolist        |\n
        | 6:Quit                  |\n
        ---------------------------\n
     "
        )
    }
    select_option(user, client)
}

fn select_option(user: &mut Option<User>, mut client: &mut Client) {
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Mauvaise saisie");
    if user.is_none() {
        match choice.as_str().trim() {
            "1" => User::login(user, &mut client),
            "2" => User::register(user, &mut client),
            _ => {
                println!("Choisissez un nombre valide");
                select_option(user, client)
            }
        }
    } else {
        match choice.as_str().trim() {
            "1" => BankAccount::create_bank_account(user, &mut client),
            "2" => User::register(user, &mut client),
            "3" => User::register(user, &mut client),
            "4" => User::register(user, &mut client),
            "5" => User::register(user, &mut client),
            "6" => User::register(user, &mut client),
            _ => {
                println!("Choisissez un nombre valide");
                select_option(user, client)
            }
        }
    }
}

fn main() {
    let mut current_user: Option<User> = None;
    let mut client = Client::connect("postgresql://bankinguser:postgres@localhost/banking", NoTls)
        .expect("No connection");
    println!("Bienvenue dans votre application bancaire !");
    show_menu(&mut current_user, &mut client);
    select_option(&mut current_user, &mut client);
}
