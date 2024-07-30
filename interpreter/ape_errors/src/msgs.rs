use crate::Error;

impl Error {
    pub fn e101(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        }
        self.panic(
            "syntax",
            101,
            format!("unknown character '{}', at {}:{}-{}", args[0], line, pos.0, pos.1),
        );
    }

    pub fn e102(&self, line: usize, pos: (usize, usize), _args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            102,
            format!("malformed or unterminated char, at {}:{}-{}", line, pos.0, pos.1),
        );
    }

    pub fn e103(&self, line: usize, pos: (usize, usize), _args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            103,
            format!("unterminated string, at {}:{}-{}", line, pos.0, pos.1),
        );
    }

    pub fn e104(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            104,
            format!(
                "failed to parse {} base number '{}', at {}:{}-{}",
                args[0], args[1], line, pos.0, pos.1
            ),
        );
    }

    pub fn e201(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            201,
            format!("unexpected token '{}', at {}:{}-{}", args[0], line, pos.0, pos.1),
        );
    }

    pub fn e202(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            202,
            format!("failed to unwrap a number '{}', at {}:{}-{}", args[0], line, pos.0, pos.1),
        );
    }

    pub fn e203(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            203,
            format!("failed to parse '{}', at {}:{}-{}", args[0], line, pos.0, pos.1),
        );
    }

    pub fn e204(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "syntax",
            204,
            format!("expeted a token '{}', at {}:{}-{}", args[0], line, pos.0, pos.1),
        );
    }
}
