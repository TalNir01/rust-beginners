struct Big {
    data: [i32; 1000], // Array, not a vector, actual data
}

impl Big {
    fn new_boxed() -> Box<Self> {
        Box::new(Self { data: [0; 1000] })
    }
}

#[derive(Debug, Clone)]
enum Event {
    PageView {
        user_id: u64,
        page: String,
    },
    Purchase {
        user_id: u64,
        order_id: u64,
        amount_cents: u64,
    },
}

trait Shape {
    fn draw(&self);
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

struct Rectangle {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle");
    }
}

fn main() {
    {
        println!("Example #1");
        dbg!(std::mem::size_of::<Big>());
        let big = Box::new(Big { data: [0; 1000] });
        dbg!(std::mem::size_of::<Box<Big>>());
    }

    {
        println!("Example #2");
        let mut v: Vec<Box<Big>> = Vec::new();
        v.push(Big::new_boxed());
        v.push(Big::new_boxed());
        v.push(Big::new_boxed());
        v.push(Big::new_boxed());

        v.insert(2, Big::new_boxed());
    }

    {
        println!("Box Example #3");
        let mut v: Box<Vec<i32>> = Box::new(vec![1, 2, 3]);
        v.push(2); // V is `box` but it's transparent for us!

        let _: &Box<Vec<i32>> = &v; // Reference to the BOX (Pointer 2 pointer, not actual data)
        let _: &Vec<i32> = v.as_ref(); // also there is `v.as_mut()`

        for item in v.as_ref() {}
        for item in *v {} // After this `v` is being "dropped"
    }

    {
        println!("Example #4");
        let mut v: Vec<Box<dyn Shape>> = Vec::new();
        v.push(Box::new(Circle {
            y: 0.,
            x: 0.,
            radius: 2.0,
        }));
        for shape in v.iter() {
            shape.draw();
        }
    }

    {}
}
