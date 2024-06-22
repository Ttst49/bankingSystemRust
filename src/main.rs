use std::io::{stdin};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use postgres;
use postgres::{Client, NoTls};
use postgres_query::{FromSqlRow, query};


#[derive(Debug)]
#[derive(FromSqlRow)]
struct User{
    id:i32,
    first_name:Option<String>,
    last_name:Option<String>,
    username:String,
    password:String
}

impl User {
    fn new(
        client:&mut Client,
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

    async fn log(mut user: &Option<User>,
                   client:&mut Client,
                   username:String,
                   mut password:String,){
        let params =
            Sha512Params::new(10_000).expect("password problem");
        password =
            sha512_simple(&*password, &params).expect("password problem");
        for row in client.query("SELECT * FROM users \
        WHERE username = $1 AND password = $2",
                       &[
                           &username,
                           &password
                       ]).expect("No return from db"){
            let user_logged_in = User{
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                username: row.get(3),
                password: row.get(4),
            };
            println!("Hey back {}",user_logged_in.username)
        }

    }

    fn register(user: &Option<User>, client: &mut Client){
        let mut first_name = String::new();
        let mut last_name = String::new();
        println!("What username for you new user ?");
        let mut username = String::new();
        stdin().read_line(&mut username).unwrap();
        println!("What password for you new user ?");
        let mut password = String::new();
        stdin().read_line(&mut password).unwrap();
        println!("Do you want to specify first name?");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        if choice.as_str().trim() == "yes" {
            println!("What first name for you new user ?");
            stdin().read_line(&mut first_name).unwrap();
        }
        println!("Do you want to specify last name?");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        if choice.as_str().trim() == "yes" {
            println!("What last name for you new user ?");
            stdin().read_line(&mut last_name).unwrap();
        }
        User::new(client, username, password, Some(first_name), Some(last_name));
        show_menu(user,client)
    }

    fn login(user: &Option<User>, client: &mut Client){
        println!("Username ?");
        let mut username = String::new();
        stdin().read_line(&mut username).unwrap();
        println!("Password ?");
        let mut password = String::new();
        stdin().read_line(&mut password).unwrap();

        User::log(user,client, username, password);
        show_menu(user,client)
    }
}

fn show_menu(user: &Option<User>,client: &Client){
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

fn select_option(user: &Option<User>, mut client: &mut Client){
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Mauvaise saisie");
    if user.is_none() {
        match choice.as_str().trim() {
            "1"=>User::login(&user,&mut client),
            "2"=>User::register(&user,&mut client),
            _ => {
                println!("Choisissez un nombre valide");
                select_option(&user,client)
            }
        }
    }else {
        match choice.as_str().trim() {
            "1"=>User::register(&user,&mut client),
            "2"=>User::register(&user,&mut client),
            "3"=>User::register(&user,&mut client),
            "4"=>User::register(&user,&mut client),
            "5"=>User::register(&user,&mut client),
            "6"=>User::register(&user,&mut client),
            _ => {
                println!("Choisissez un nombre valide");
                select_option(&user,client)
            }
        }
    }
}

fn main() {
    let current_user:Option<User> = None;
    let mut client =
        Client::connect("postgresql://bankinguser:postgres@localhost/banking",NoTls)
            .expect("No connection");
    println!("Bienvenue dans votre application bancaire !");
    show_menu(&current_user,&client);
    select_option(&current_user,&mut client);
}
