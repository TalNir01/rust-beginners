fn main() {
    {
        println!("Tuples Example");
        let t: (bool, (f64, f64)) = (true, (2.5, 2.6));
        let x: f64 = t.1.0;
        let y: f64 = t.1.1;
        println!("x: {}, y: {}", x, y);

        let (flag, (x, y)) = t;
        println!("flag: {}, x: {}, y: {}", flag, x, y);
    }

    {
        println!("Condition / Expression Example");
        let x = {
            let y = 10;
            let z = 15;
            if y > z { y + z } else { y - z } // Return!
        };
        println!("x: {}", x);
    }

    {
        println!("Match Example");
        let num = 5;
        match num {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("Other"),
        }; // Match is an expression, so we need a semicolon here, `match condition {options => }`
    }
}
