use crate::Error;

impl Error {
    pub fn e101(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!("unknown character '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }

    pub fn e102(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!(
                "malformed or unterminated char '{}', at line {}",
                args[0], line
            ),
        );
        self.print_lines(line);
    }

    pub fn e103(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!("unterminated string '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }

    pub fn e104(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!(
                "failed to parse {} base number '{}', at line {}",
                args[0], args[1], line
            ),
        );
        self.print_lines(line);
    }

    pub fn e201(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!("unexpected token '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }

    pub fn e202(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!("failed to unwrap a number '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }

    pub fn e203(&self, line: usize, args: Vec<String>) {
        self.panic(
            "syntax",
            101,
            format!("failed to parse '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }

    pub fn e204(&self, line: usize, args: Vec<String>) {
        self.eprintln(
            "syntax",
            101,
            format!("expeted a token '{}', at line {}", args[0], line),
        );
        self.print_lines(line);
    }
}
