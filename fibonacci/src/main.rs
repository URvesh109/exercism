fn create_buffer(size: usize) -> Vec<usize> {
    vec![0; size]
}

struct Fobonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fobonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fib = Fobonacci { curr: 0, next: 1 };

    for i in fib.take(8) {
        println!("{i}");
    }
}
