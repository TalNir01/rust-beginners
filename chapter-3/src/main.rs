fn print_vec(v: Vec<i32>) {
    dbg!(v); // Takes ownership of v and prints it to the console -> After it `v` is no longer valid
}

fn modify_vec(v: &mut Vec<i32>) {
    v.push(10); // Modifies the vector by adding a new element
    dbg!(v); // Prints the modified vector to the console
}

// Function to calculate the sum of elements in a vector
// Takes a reference to a vector of i32 and returns the sum of its elements
// The function uses a for loop to iterate over the elements of the vector and adds them to a sum variable, which is returned at the end
// The function uses the dereference operator (*) to access the value of each element in the vector, since the elements are references to i32 values
// Arguments:
// - v: A reference to a vector of i32 values
// Returns:
// - The sum of the elements in the vector as an i32 value
fn sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for ele in v.iter() {
        sum += *ele; // Dereference the reference to get the value
    }
    sum
}

// Use `slice` instead of `&Vec<i32>` to make the function more flexible and accept any slice of i32 values, not just vectors
fn sum2(v: &[i32]) -> i32 {
    let mut sum = 0;
    for ele in v.iter() {
        sum += *ele; // Dereference the reference to get the value
    }
    sum
}

fn multiply(v: &mut [i32], factor: i32) {
    for ele in v.iter_mut() {
        *ele *= factor; // Dereference the reference to get the value and multiply it by the factor
    }
}

fn get_statistics(v: &[i32]) -> (i32, i32, f32) {
    let mut num_positive = 0;
    let mut num_negative = 0;
    let mut sum = 0;

    for ele in v.iter() {
        if *ele > 0 {
            num_positive += 1;
        } else if *ele < 0 {
            num_negative += 1;
        }
        sum += *ele;
    }
    (num_positive, num_negative, sum as f32 / v.len() as f32)
}

fn main() {
    {
        println!("Vector Example");
        // Option 1 - Explicit vec declaration
        let mut v = Vec::new();
        v.push(1); // v: [1]
        v.push(2);
        print_vec(v); // Takes ownership of v and prints it to the console -> After it `v` is no longer valid

        // Option 2 - Using the vec! macro
        let mut v = vec![1, 2, 3]; // Support 2 syntaxes: vec![1, 2, 3] or vec![1; 3] -> Creates a vector of 3 elements, all initialized to 1
        v.push(-10);
        print_vec(v); // Takes ownership of v and prints it to the console -> After it `v` is no longer valid
        // println!("Length of vector is {}", v.len()); // This will cause a compile-time error because `v` has been moved to `print_vec` and is no longer valid");
    }
    {
        println!("Vector Ownership Example");
        let v = vec![6; 10]; // Creates a vector of 10 elements, all initialized to 6
        // Takes ownership of v and prints each element to the console -> After it `v` is no longer valid
        for ele in v {
            println!("{}", ele);
        }
        // Reprint
        // dbg!(v); // This will cause a compile-time error because `v` has been moved to the for loop and is no longer valid

        // Fix by `.iter()` method
        let v = vec![6; 10]; // Creates a vector of 10 elements, all initialized to 6
        // By using `.iter()` method, we can iterate over the elements of the vector without taking ownership of it, so `v` remains valid after the loop
        for ele in v.iter() {
            println!("{}", ele);
        }
        dbg!(v); // This will work because `v` is still valid after the for loop

        // Fix by `.iter()` method
        let v = vec![6; 10]; // Creates a vector of 10 elements, all initialized to 6
        // By using `.iter()` method, we can iterate over the elements of the vector without taking ownership of it, so `v` remains valid after the loop
        for ele in &v {
            println!("{}", ele);
        }
        dbg!(v); // This will work because `v` is still valid after the for loop
    }

    {
        println!("Reference Example");
        let x: i32 = 2;
        let x_ref: &i32 = &x; // x_ref is a reference to x

        println!("x: {}, x_ref: {}, manual-dereference: {}", x, x_ref, *x_ref); // This will work because x is still valid after the reference is created
    }

    {
        println!("Reference Example #2");
        let mut x: i32 = 2;
        let x_ref: &mut i32 = &mut x; // x_ref is a reference to x (mutable reference)
        dbg!(*x_ref); // This will work because x is still valid after the reference is created
        *x_ref += 1; // This will work because x is still valid after the reference is created, mutating original value!
        dbg!(*x_ref); // This will work because x is still valid after the reference is created
        // NOTE: To change what `x_ref` you must redeclare as mutable `let mut x_ref: &mut i32 = &mut x;`
        // let mut y = 10;
        // x_ref = &mut y; // Raise an compile error, due to the variable `x_ref` is not mutable, so we cannot reassign it to a new reference. To fix this, we can declare `x_ref` as mutable by using `let mut x_ref: &mut i32 = &mut x;` instead of `let x_ref: &mut i32 = &mut x;`
        // dbg!(x_ref); // This will work because x is still valid after the reference is created
    }

    {
        println!("Modify Vector Example");
        let mut v = vec![1, 2, 3]; // Creates a vector
        modify_vec(&mut v); // Passes a mutable reference to the vector to the function
        // modify_vec(&v); // Error: function expect mutable reference, but we are passing an immutable reference. To fix this, we can pass a mutable reference to the function by using `&mut v` instead of `&v` (Types differ in mutability)
        dbg!(v); // This will work because `v` is still valid after the function call
    }

    {
        println!("Modify Vector Example #2");
        let mut v = vec![2]; // Creates a vector
        // let r = &v[0]; // Creates a reference to the first element of the vector
        modify_vec(&mut v); // Passes a mutable reference to the vector to the function
        // dbg!(*r); // Cannot use mutable and immutable references in the same scope, so this will cause a compile-time error because `r` is an immutable reference to the first element of the vector, and `modify_vec` takes a mutable reference to the vector. To fix this, we can either remove the immutable reference or use a mutable reference instead.
        dbg!(v); // This will work because `v` is still valid after the function call
    }

    {
        println!("Slice Borrowing Example");
        let v = vec![1, 2, 3, 4, 5, 6]; // Creates a vector
        let slice = &v[1..3]; // Creates a mutable slice of the
        dbg!(slice); // This will work because `slice` is a reference to the first two elements of the vector, and `v` is still valid after the slice is created
    }

    {
        println!("Sum Example");
        let v = vec![1, 2, 3, 4, 5]; // Creates a vector
        // let sum = sum(&v[0..3]); // Compile Error - Type `&[i32]` cannot be passed to a function that expects `&Vec<i32>`.
        let sum = sum(&v); // Passes a reference to the vector to the function
        dbg!(sum); // This will work because `v` is still valid after the function call
    }

    {
        println!("Sum Example #2");
        let v = vec![1, 2, 3, 4, 5]; // Creates a vector
        let sum1 = sum2(&v[0..3]); // Works!
        let sum2 = sum2(&v); // Passes a reference to the vector to the function
        dbg!(sum1, sum2); // This will work because `v` is still valid after the function call

        // NOTE: Every vector can be represented implicit as slice (default behavior), so we can pass a vector to a function that expects a slice without needing to explicitly create a slice. This is because a vector is essentially a contiguous block of memory that can be treated as a slice, and Rust automatically converts the vector to a slice when needed.
        // NOTE: We can use `iter_mut` instead of `iter` to iterate over the elements of a vector and modify them in place. This is useful when we want to change the values of the elements in the vector without creating a new vector. The `iter_mut` method returns an iterator that yields mutable references to the elements of the vector, allowing us to modify them directly.
    }

    {
        println!("Multiply Example");
        let mut v = vec![1, 2, 3, 4, 5]; // Creates a vector
        multiply(&mut v, 2); // Passes a mutable reference to the vector and a factor to the function
        multiply(&mut v[..], 2); // Same with slice of the same object
        dbg!(v); // This will work because `v` is still valid after the function call.
    }

    {
        println!("Statistics Example");
        let v = vec![1, 2, 3, -4, -5]; // Creates a vector
        let (num_positive, num_negative, average) = get_statistics(&v); // Passes a reference to the vector to the function
        dbg!(num_positive, num_negative, average); // This will work because `v` is still valid after the function call
    }
}

#[test]
fn test_get_statistics() {
    let v = vec![1, 2, 3, -4, -5]; // Creates a vector
    let (num_positive, num_negative, average) = get_statistics(&v); // Passes a reference to the vector to the function
    assert_eq!(num_positive, 3); // Asserts that the number of positive elements is 3
    assert_eq!(num_negative, 2); // Asserts that the number of negative elements is 2
    assert_eq!(average, -0.6); // Asserts that the average of the elements is -0.6
}
