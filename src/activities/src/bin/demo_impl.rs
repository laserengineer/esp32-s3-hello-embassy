// use impl for temperature struct with f64 data type

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0124 }
    }

    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }

    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }

    fn print_message(&self, message: &str) {
        println!("{}: {:.2} degrees F", message, self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.134 };
    // Temperature::show_temp(hot);
    hot.show_temp();
    hot.print_message("Hot");

    let cold = Temperature::freezing();
    cold.show_temp();
    cold.print_message("Cold");

    let boiling = Temperature::boiling();
    boiling.show_temp();
    boiling.print_message("Boiling");
}
