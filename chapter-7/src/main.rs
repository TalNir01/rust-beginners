struct User {
    id: u32,
    username: String,
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Self { users: Vec::new() }
    }

    fn find_user_by_id(&self, id: u32) -> Option<&User> {
        for user in self.users.iter() {
            if user.id == id {
                return Some(user);
            }
        }
        None
    }
}

#[derive(Debug)]
enum MyError {
    DivisionByZero,
}

fn divide(a: f64, b: f64) -> Result<f64, MyError> {
    if b == 0.0 {
        Err(MyError::DivisionByZero)
    } else {
        Ok(a / b) // Returns Ok (f64) if the division is successful
    }
}

fn main() {
    {
        println!("Options Example 1:");
        let db = Database::new(); // create db object
        let user: Option<&User> = db.find_user_by_id(1);
        // if let Some(u) = user {
        //     println!("User found: {}", u.username);
        // } else {
        //     println!("User not found");
        // }
        match user {
            Some(u) => println!("User found: {}", u.username),
            None => eprintln!("User not found"),
        }

        let maybe_num = Some(42); // None;
        dbg!(maybe_num.is_some());
        dbg!(maybe_num.is_none());

        dbg!(maybe_num.unwrap_or(0)); // unwrap_or returns the value inside the Option if it is Some, otherwise it returns the default value provided (0 in this case).
        dbg!(maybe_num.unwrap()); // unwrap will panic if the Option is None, so this line will cause a panic since maybe_num is None.
    }

    {
        println!("Options Example 2:");
        let maybe_num: Option<i32> = Some(42);
        let maybe_num_ref: Option<&i32> = maybe_num.as_ref(); // as_ref converts Option<T> to Option<&T>

        let mut maybe_num2: Option<i32> = Some(42);
        let maybe_num_ref2: Option<&mut i32> = maybe_num2.as_mut(); // as_mut converts Option<T> to Option<&mut T>
        if let Some(num) = maybe_num_ref2 {
            *num += 1; // modify the value inside the Option
        }
        dbg!(maybe_num2); // prints Some(43)
    }

    {
        println!("Results Example:");
        let result = divide(10.0, 2.0);
        match result {
            Ok(value) => println!("Result: {:?}", value),
            Err(error) => eprintln!("Error: {:?}", error),
        }

        let result = divide(10.0, 0.0);
        match result {
            Ok(value) => println!("Result: {:?}", value),
            Err(error) => eprintln!("Error: {:?}", error),
        }
    }
}
