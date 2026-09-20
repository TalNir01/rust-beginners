mod utils {
    //node Must use `pub` to make the function public
    pub fn foo() -> i32 {
        println!("called `utils::foo()`");
        10
    }

    pub(crate) mod math {
        pub(crate) fn add(x: i32, y: i32) -> i32 {
            println!("called `utils::math::add()`");
            x + y
        }
    }
}

mod db {
    pub(crate) struct Connection {}

    pub(crate) struct Database {
        connection: Connection,
    }

    impl Database {
        pub(crate) fn new() -> Self {
            dbg!("called `db::Database::new()`");
            Self {
                connection: Connection {},
            }
        }

        pub(crate) fn connection(&self) -> &Connection {
            &self.connection
        }
    }
}

fn main() {
    {
        println!("Mods Example");
        let result = utils::foo();
        println!("Result from utils::foo(): {}", result);
        println!(
            "Result from utils::math::add(): {}",
            utils::math::add(5, 10)
        );
    }

    {
        let d = db::Database::new();
        // d.connection; // Don't work because connection is private
        d.connection(); // Works fine
    }
}
