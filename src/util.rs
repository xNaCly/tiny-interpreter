pub struct Logger {
    prefix_warn: String,
    prefix_info: String,
    prefix_panic: String,
    prefix_debug: String,
    pub print_debug: bool,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            prefix_warn: String::from("warn: "),
            prefix_info: String::from("inf: "),
            prefix_panic: String::from("pan: "),
            prefix_debug: String::from("dbg: "),
            print_debug: false,
        }
    }

    pub fn warn(&self, msg: &str) {
        println!("{}{}", self.prefix_warn, msg);
    }

    pub fn info(&self, msg: &str) {
        println!("{}{}", self.prefix_info, msg);
    }

    pub fn panic(&self, msg: &str){
        println!("{}{}", self.prefix_info, msg);
        panic!("{}", msg);
    }

    pub fn debug(&self, msg: &str) {
        if self.print_debug {
            println!("{}{}", self.prefix_debug, msg);
        }
    }
}
