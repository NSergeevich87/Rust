#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    phone: String,
    email: Option<String>,
    //email: String,
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn change_email(&mut self, email: String) {
        self.email = email;
    }
}

struct Point (f64, f64, f64);

fn main() {
    let nikita = Person {
        first_name: String::from("Nikita"),
        last_name: String::from("Popov"),
        age: 32,
        phone: String::from("555-1234"),
        //email: None,
        email: Some("".to_string()),
        //email: "email@mail.ru".to_string(),
    };

    println!("{:?}", nikita);

    let mut user = User::new(
        String::from("Daniil"),
        String::from("daniil@mail.ru"),
        String::from("http://daniil.com"),
    );
    user.deactivate();
    println!("Hello! {:?}", user);
    println!("user's email: {}", user.email);
    user.change_email("new_email@daniil.com".to_string());
    println!("user's email: {}", user.email);

    let username = "Nikita".to_string();
    let email = "email".to_string();
    let uri = "http://nikita.com".to_string();
    let active = true;
    let new_user = User { username, email, uri, active };
    println!("{:?}", new_user);

    let point = Point(1.0, 2.45, 3.0);
    println!("second point: {}", point.1);
}