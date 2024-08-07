use crate::show_menu;
use postgres::Client;
pub use postgres_query::FromSqlRow;
pub use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
pub use std::error::Error;
pub use std::io::stdin;

#[derive(Debug, FromSqlRow)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub password: String,
}

impl User {
    fn new(
        client: &mut Client,
        username: String,
        mut password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> User {
        if !password.is_empty() {
            let params = Sha512Params::new(10_000).expect("Random error");
            password = sha512_simple(&*password, &params).expect("Should not fail");
        }
        let mut user = User {
            id: 0,
            first_name: None,
            last_name: None,
            username,
            password,
        };
        if let Some(payload) = first_name {
            user.first_name = Some(payload.to_string())
        } else {
            user.first_name = Some("undefined".to_string())
        }
        if let Some(payload) = last_name {
            user.last_name = Some(payload.to_string())
        } else {
            user.last_name = Some("undefined".to_string())
        }
        if !&user.username.is_empty() {
            client
                .execute(
                    "INSERT INTO users\
                 (username,password,last_name,first_name) \
                 VALUES ($1,$2,$3,$4)",
                    &[
                        &user.username.trim(),
                        &user.password.trim(),
                        &user.last_name,
                        &user.first_name,
                    ],
                )
                .expect("Everything panicked");
        }
        user
    }

    fn log_into(
        client: &mut Client,
        username: String,
        password: String,
    ) -> Result<(), Box<dyn Error>> {
        let row = client.query_opt(
            "SELECT * FROM users WHERE username = $1",
            &[&username.trim()],
        )?;

        match row {
            Some(row) => {
                let db_id = row.get("id");
                let db_username: &str = row.get("username");
                let db_password: &str = row.get("password");
                let db_first_name: &str = row.get("first_name");
                let db_last_name: &str = row.get("last_name");

                match sha512_check(&password, db_password) {
                    Ok(()) => {
                        println!("Hello, {}", db_username);
                        let current_user = User {
                            id: db_id,
                            first_name: Some(db_first_name.to_string()),
                            last_name: Some(db_last_name.to_string()),
                            username: db_username.to_string(),
                            password: db_password.to_string(),
                        };
                        show_menu(&mut Some(current_user), client)
                    }
                    Err(_) => {
                        println!("Invalid username or password");
                    }
                }
            }
            None => {
                println!("Invalid username or password");
            }
        }
        Ok(())
    }

    pub fn register(user: &mut Option<User>, client: &mut Client) {
        let mut first_name = String::new();
        let mut last_name = String::new();
        println!("What username for you new user ?");
        let mut username = String::new();
        stdin().read_line(&mut username).unwrap();
        println!("What password for you new user ?");
        let mut password = String::new();
        stdin().read_line(&mut password).unwrap();
        println!("Do you want to specify first name?(yes/no)");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        if choice.as_str().trim() == "yes" {
            println!("What first name for you new user ?");
            stdin().read_line(&mut first_name).unwrap();
        }
        println!("Do you want to specify last name?(yes/no)");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        if choice.as_str().trim() == "yes" {
            println!("What last name for you new user ?");
            stdin().read_line(&mut last_name).unwrap();
        }
        User::new(
            client,
            username,
            password,
            Some(first_name),
            Some(last_name),
        );
        show_menu(user, client)
    }

    pub fn login(user: &mut Option<User>, client: &mut Client) {
        println!("Username ?");
        let mut username = String::new();
        stdin().read_line(&mut username).unwrap();
        println!("Password ?");
        let mut password = String::new();
        stdin().read_line(&mut password).unwrap();

        User::log_into(client, username, password).expect("something's wrong");
        show_menu(user, client)
    }
}
