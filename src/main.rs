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
           username:String,
           password:String,
           first_name:Option<String>,
           last_name:Option<String>
    )->User{
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
        user
    }
}

fn show_menu(){}

fn main() {
    let user = User::new(
        String::from("tibo"),
        String::from("password"),
        Some(String::from("Thibaut")),
        None
    );
    println!("{:?}",user)
}
