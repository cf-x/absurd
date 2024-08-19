use super::Error;

impl Error {
    pub fn e101(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            101,
            format!(
                "unknown character '{}', at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e102(&self, line: usize, pos: (usize, usize), _args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            102,
            format!(
                "malformed or unterminated char, at {}:{}-{}",
                line, pos.0, pos.1
            ),
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
            format!(
                "unexpected token '{}', at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e202(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            202,
            format!(
                "failed to unwrap a number '{}', at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e203(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "syntax",
            203,
            format!(
                "failed to parse '{}', at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e204(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "syntax",
            204,
            format!(
                "expeted a token '{}', at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e301(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "runtime",
            301,
            format!(
                "type mismatch: expected '{}', got {}, at {}:{}-{}",
                args[0], args[1], line, pos.0, pos.1
            ),
        );
    }

    pub fn e302(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.panic(
            "runtime",
            302,
            format!(
                "break statement not within a loop, at {}:{}-{}",
                line, pos.0, pos.1
            ),
        );
    }

    pub fn e303(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            303,
            format!(
                "return statement not within a function, at {}:{}-{}",
                line, pos.0, pos.1
            ),
        );
    }

    pub fn e304(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            304,
            format!(
                "await statement not within an async function, at {}:{}-{}",
                line, pos.0, pos.1
            ),
        );
    }

    pub fn e305(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            305,
            format!(
                "invalid function return type, at {}:{}-{}",
                line, pos.0, pos.1
            ),
        );
    }

    pub fn e306(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            306,
            format!(
                "failed to resolve {}, at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e307(&self, line: usize, pos: (usize, usize), args: Vec<String>) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            307,
            format!(
                "{} is already declared, at {}:{}-{}",
                args[0], line, pos.0, pos.1
            ),
        );
    }

    pub fn e308(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            308,
            format!("stack underflow, at {}:{}-{}", line, pos.0, pos.1),
        );
    }

    pub fn e309(&self, line: usize, pos: (usize, usize)) {
        if pos != (0, 0) {
            self.print_lines(line, pos);
        }
        self.eprintln(
            "runtime",
            309,
            format!("stack overflow, at {}:{}-{}", line, pos.0, pos.1),
        );
    }
}
