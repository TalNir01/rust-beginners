#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        ((self.x.powi(2) + self.y.powi(2)) as f64).sqrt()
    }

    fn move_point(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Authentication {
    NoAuth,
    Token(String),
    UsernamePassword { username: String, password: String },
}

impl Authentication {
    fn is_authenticated(&self) -> bool {
        match self {
            Authentication::NoAuth => false,
            Authentication::Token(token) => token == "secret_token",
            Authentication::UsernamePassword {
                username: un,
                password: pswd,
            } => un == "user" && pswd == "pass",
        }
    }
}

fn handle_request(auth: Authentication) -> bool {
    match auth {
        Authentication::NoAuth => {
            println!("No authentication provided.");
            false
        }
        Authentication::Token(token) => {
            println!("Authenticating with token: {}", token);
            true
        }
        Authentication::UsernamePassword { username, password } => {
            println!(
                "Authenticating with username: {} and password: {}",
                username, password
            );
            true
        }
    }
}

fn main() {
    {
        println!("Structs Example");
        let mut p = Point { x: 5.0, y: 10.0 };
        println!("Point: ({}, {})", p.x, p.y);
        p.x += 1.0;
        println!("Point after modification: ({}, {})", p.x, p.y);
    }

    {
        println!("Enum Example");
        let color = Color::Red;
        println!("Color: {:#?}", color);
        match color {
            Color::Red => println!("The color is Red"),
            Color::Green => println!("The color is Green"),
            Color::Blue => println!("The color is Blue"),
        }
    }

    {
        println!("Enum Example #2");
        let auth_types = vec![
            Authentication::NoAuth,
            Authentication::Token("secret_token".into()),
            Authentication::UsernamePassword {
                username: "user".into(),
                password: "pass".into(),
            },
        ];
        for auth in auth_types {
            dbg!(handle_request(auth));
        }
    }

    {
        println!("Enum Example #3");
        let auths_list = vec![
            Authentication::NoAuth,
            Authentication::Token("secret_token".into()),
            Authentication::UsernamePassword {
                username: "user".into(),
                password: "pass".into(),
            },
            Authentication::Token("wrong_token".into()),
            Authentication::UsernamePassword {
                username: "wrong_user".into(),
                password: "wrong_pass".into(),
            },
        ];

        for auth in auths_list {
            dbg!(auth.is_authenticated());
        }
    }
}
