pub struct Verbose {
    verbose: bool,
}

impl Verbose {
    pub fn dump_verbose(&self, s: &String) {
        if self.verbose {
            println!("{}", s);
        }
    }
    pub fn new(verbose: bool) -> Verbose {
        Verbose { verbose }
    }
}
