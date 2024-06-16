use std::io::{Read, stdin};
use sha_crypt::{sha512_simple, Sha512Params};
use postgres;
use postgres::{Client, NoTls};


#[derive(Debug)]
struct User{
    id:i32,
    first_name:Option<String>,
    last_name:Option<String>,
    username:String,
    password:String
}

impl User {
    fn new(
        mut client: Client,
        username:String,
        mut password:String,
        first_name:Option<String>,
        last_name:Option<String>
    ) ->User{
        if !password.is_empty() {
            let params =
                Sha512Params::new(10_000).expect("Random error");
            password =
                sha512_simple(&*password, &params).expect("Should not fail");
        }
        let mut user = User{
            id: 0,
            first_name: None,
            last_name: None,
            username,
            password
        };
        if let Some(payload) = first_name{
            user.first_name = Some(payload.to_string())
        }else {
            user.first_name = Some("undefined".to_string())
        }
        if let Some(payload) = last_name{
            user.last_name = Some(payload.to_string())
        }else {
            user.last_name = Some("undefined".to_string())
        }
        if !&user.username.is_empty(){
            client.execute(
                "INSERT INTO users\
                 (username,password,last_name,first_name) \
                 VALUES ($1,$2,$3,$4)",
                &[
                    &user.username,
                    &user.password,
                    &user.last_name,
                    &user.first_name
                ]
            ).expect("Everything panicked");
        }
        user
    }
}

fn show_menu(user: &Option<User>){
    if user.is_none() {
        println!(
            "\n
        --------------------\n
        | 1:Login          |\n
        | 2:Register       |\n
        --------------------\n
     "
        )
    }else {
        println!(
            "\n
        --------------------\n
        | 1:Add a todo     |\n
        | 2:Remove a todo  |\n
        | 3:Edit a todo    |\n
        | 4:Show todolist  |\n
        | 5:Reset todolist |\n
        | 6:Quit           |\n
        --------------------\n
     "
        )
    }

}

fn select_option(user: &Option<User>){
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Mauvaise saisie");
    if user.is_none() {
        match choice.as_str().trim() {
            "1"=>show_menu(&user),
            _ => {
                println!("Choisissez un nombre valide");
                select_option(&user)
            }
        }
    }
}

fn main() {
    let current_user:Option<User> = None;
    let client =
        Client::connect("postgresql://bankinguser:postgres@localhost/banking",NoTls)
            .expect("No connection");
    println!("Bienvenue dans votre application bancaire !");
    show_menu(&current_user);
    select_option(&current_user);

}
