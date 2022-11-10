struct MultiplicationTable {
    size: isize,
}

impl MultiplicationTable {
    fn new(size: isize) -> Self {
        MultiplicationTable { size }
    }

    fn print(&self) {
        print!("|     |");
        for i in 0..=self.size {
            Self::print_number(&i);
        }

        print!("\n");

        for a in 0..=self.size {
            print!("|");
            Self::print_number(&a);
            for b in 0..=self.size {
                Self::print_number(&(a * b));
            }
            print!("\n");
        }
    }

    fn print_number(n: &isize) {
        print!("{:^5}|", n);
    }
}

fn main() {
    let table = MultiplicationTable::new(12);

    table.print();
}
