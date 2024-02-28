#[derive(Debug)]
pub enum Token<'s> {
    LabelDef(&'s str),
    LabelRef(&'s str),
    String(&'s str),
    Number(i32),
    Print,
    In,
    Out,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Inc,
    Dec,
    And,
    Or,
    Not,
    Xor,
    Shl,
    Shr,
    Pop,
    Dup,
    Swp,
    Ovr,
    Load,
    Stor,
    Jmp,
    Je,
    Jne,
    Jg,
    Jge,
    Jl,
    Jle,
    Nop,
    Halt,
    Rf,
    Crf,
}

pub struct Lexer<'s> {
    source: &'s str,
    current: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Lexer { source, current: 0 }
    }

    pub fn tokenize(self) -> Vec<Token<'s>> {
        self.collect()
    }

    fn next_token(&mut self) -> Option<Token<'s>> {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.consume();
                continue;
            }
            if c == ';' {
                self.consume_until_newline();
                self.consume();
                continue;
            }

            if c.is_ascii_alphabetic() {
                return Some(self.tokenize_instruction());
            }
            if c.is_ascii_digit() {
                return Some(self.tokenize_number());
            }
            if c == ':' {
                return Some(self.tokenize_label_def());
            }
            if c == '@' {
                return Some(self.tokenize_label_ref());
            }
            if c == '"' {
                return Some(self.tokenize_string());
            }
            panic!("unexpected char: {c}");
        }
        None
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn consume(&mut self) -> Option<char> {
        if let Some(c) = self.source[self.current..].chars().next() {
            self.current += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn consume_until<P>(&mut self, mut pred: P)
    where
        P: FnMut(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if pred(c) {
                break;
            } else {
                self.consume();
            }
        }
    }

    fn consume_until_whitespace(&mut self) {
        self.consume_until(|c| c.is_ascii_whitespace());
    }

    fn consume_until_newline(&mut self) {
        self.consume_until(|c| c == '\n');
    }

    fn tokenize_label_def(&mut self) -> Token<'s> {
        self.consume(); // the ':'
        let start = self.current;
        self.consume_until_whitespace();
        let end = self.current;
        Token::LabelDef(&self.source[start..end])
    }

    fn tokenize_label_ref(&mut self) -> Token<'s> {
        self.consume(); // the '@'
        let start = self.current;
        self.consume_until_whitespace();
        let end = self.current;
        Token::LabelRef(&self.source[start..end])
    }

    fn tokenize_string(&mut self) -> Token<'s> {
        self.consume(); // the opening "
        let start = self.current;
        self.consume_until(|c| c == '"');
        let end = self.current;
        self.consume(); // the closing "
        Token::String(&self.source[start..end])
    }

    fn tokenize_number(&mut self) -> Token<'s> {
        let start = self.current;
        self.consume_until_whitespace();
        let end = self.current;
        let slice = &self.source[start..end];
        let num = slice
            .parse()
            .expect(&format!("unable to parse number '{slice}'"));
        Token::Number(num)
    }

    fn tokenize_instruction(&mut self) -> Token<'s> {
        let start = self.current;
        self.consume_until_whitespace();
        let end = self.current;
        let slice = &self.source[start..end];
        match slice.to_uppercase().as_str() {
            "PRINT" => Token::Print,
            "IN" => Token::In,
            "OUT" => Token::Out,
            "ADD" => Token::Add,
            "SUB" => Token::Sub,
            "MUL" => Token::Mul,
            "DIV" => Token::Div,
            "MOD" => Token::Mod,
            "NEG" => Token::Neg,
            "INC" => Token::Inc,
            "DEC" => Token::Dec,
            "AND" => Token::And,
            "OR" => Token::Or,
            "NOT" => Token::Not,
            "XOR" => Token::Xor,
            "SHL" => Token::Shl,
            "SHR" => Token::Shr,
            "POP" => Token::Pop,
            "DUP" => Token::Dup,
            "SWP" => Token::Swp,
            "OVR" => Token::Ovr,
            "LOAD" => Token::Load,
            "STOR" => Token::Stor,
            "JMP" => Token::Jmp,
            "JE" => Token::Je,
            "JNE" => Token::Jne,
            "JG" => Token::Jg,
            "JGE" => Token::Jge,
            "JL" => Token::Jl,
            "JLE" => Token::Jle,
            "NOP" => Token::Nop,
            "HALT" => Token::Halt,
            "RF" => Token::Rf,
            "CRF" => Token::Crf,
            _ => panic!("invalid instruction: '{slice}'"),
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
