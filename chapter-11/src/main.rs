struct PythonRange {
    current: u32,
    start: u32,
    stop: u32,
    step: u32,
}

fn sum_numbers(nums: impl Iterator<Item = u32>) -> u32 {
    nums.into_iter().sum()
}

fn sum_odd_indices(nums: &[i32]) -> i32 {
    (1..)
        .zip(nums.iter())
        .filter(|(index, _)| index % 2 != 0)
        .map(|(_, item)| item)
        .sum()
}

fn sum_odd_indices_2(nums: &[i32]) -> i32 {
    nums.iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, item)| item)
        .sum()
}

impl Iterator for PythonRange {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.current >= self.stop {
            None // Stop iterator
        } else {
            let returned = Some(self.current);
            self.current += self.step;
            returned
        }
    }
}

fn range(start: u32, stop: u32, step: u32) -> PythonRange {
    PythonRange {
        current: start,
        start,
        stop,
        step,
    }
}

fn main() {
    {
        println!("Example #1");
        for num in range(0, 10, 1) {
            println!("{num}");
        }
    }

    {
        println!("Example #2");
        let i = [0, 1, 0, 1];
        dbg!(sum_odd_indices(&i)); // Starts from `1`
        dbg!(sum_odd_indices_2(&i)); // Starts from `0` - correct!!
    }
}
