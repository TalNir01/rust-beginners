fn foo2(count_until: u32) -> String {
    for i in 0..count_until {
        std::thread::sleep(std::time::Duration::from_millis(100));
        dbg!(i);
    }
    String::from("Done!")
}

fn foo() -> String {
    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        println!("Loop {i}");
    }
    String::from("done")
}
fn main() {
    {
        println!("Example #1");
        // foo(); // Has an output

        let handle1 = std::thread::spawn(foo); // They don't have enough time to run.
        let handle2 = std::thread::spawn(foo);
        let res = handle1.join().expect("All good");
        let res2 = handle2.join();
        // NOTE: We can move the `spawn` a closure
    }

    {
        println!("Example #2");
        let handle = std::thread::spawn(|| foo2(50));
        let res = handle.join().expect("Thread panicked!");
    }
}
