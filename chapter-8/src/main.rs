use rust_decimal::Decimal;

struct LabeledData<T> {
    label: String,
    data: T,
}

impl<T> LabeledData<T> {
    fn new(label: String, data: T) -> Self {
        Self { label, data }
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn data(&self) -> &T {
        &self.data
    }
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

struct Rect {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

trait HasArea {
    fn area(&self) -> f32;
}

impl HasArea for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powf(2.0) // Area of a circle is π * r^2
    }
}

impl HasArea for Rect {
    fn area(&self) -> f32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }
}

fn print_area<T>(shape: T)
where
    T: HasArea,
{
    println!("Shape area={}", shape.area())
}

trait OutputStream {
    fn output_string(&self, s: &str);
    fn output_string_and_newline(&self, s: &str) {
        self.output_string(s);
        self.output_string("\n");
    }
}

// Unit structs
struct StdoutStream;

impl OutputStream for StdoutStream {
    fn output_string(&self, s: &str) {
        print!("{s}");
    }
}

struct Square {}
trait Shape {}
impl Shape for Circle {}
impl Shape for Square {}

fn foo<S1, S2>(a: &S1, b: &S2)
where
    S1: Shape,
    S2: Shape,
{
    let mut x: &dyn Shape = a;
    x = b;
}

mod account;
mod payments;
fn main() {
    {
        println!("Generic Types #1");
        let labeled: LabeledData<[i32; 3]> = LabeledData::new("label".into(), [1, 2, 3]);
        println!("Label: {}", labeled.label());
        println!("Data: {:?}", labeled.data());
    }
    {
        println!("Traits #2");
        let circle = Circle {
            x: 0.0,
            y: 0.0,
            radius: 5.0,
        };
        let rect = Rect {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
        };
        print_area(circle);
        print_area(rect);
    }

    {
        println!("Example #3 - Account");
        let mut acct = account::Account::new();

        acct.deposit(100);

        if acct.pay(Decimal::from(10)) {
            println!("Payment Successful!")
        } else {
            eprintln!("Payment Failed")
        }
    }
}
