fn take_vec_by_value(v: Vec<i32>) {}
fn main() {
    {
        println!("Closure Example #1");

        let callback = || {
            println!("Hello");
        };

        callback();
        callback();
        callback();
    }

    {
        println!("Closure Example #2");

        let mul = |num: i32| -> i32 { num * 2 };

        dbg!(mul(2));
    }

    {
        println!("Closure Example #3");

        let mut v = vec![1, 2, 3];

        let closure = || {
            for item in v.iter() {
                println!("{item}");
            }
        }; // Immutable borrow initialized and "died"

        closure();
        v.push(100); // Used mutable borrow
        // closure(); // Used immutable borrow - Compile Error
        dbg!(v);
    }

    {
        println!("Closure Example #3");
        let mut v = vec![1, 2, 3];

        let mut closure = || {
            v.push(2);
        };

        closure();
        dbg!(v);
    }

    {
        println!("Closure Example #4");
        let mut v = vec![1, 2, 3];

        let mut closure1 = || {
            take_vec_by_value(v);
        };

        let mut closure2 = closure1.clone();

        closure1();
        closure2();
    }
}
