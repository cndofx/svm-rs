use std::collections::HashMap;

use svm::instructions::*;

use crate::token::Token;

struct LabelInfo {
    pub addr: Option<usize>,
    pub refs: Vec<usize>,
}

impl LabelInfo {
    fn new() -> Self {
        LabelInfo {
            addr: None,
            refs: Vec::new(),
        }
    }
}

pub fn generate<'s>(tokens: &[Token<'s>]) -> Vec<i32> {
    let mut code: Vec<i32> = Vec::new();
    let mut labels: HashMap<&'s str, LabelInfo> = HashMap::new();

    for tk in tokens {
        match tk {
            Token::LabelDef(name) => {
                if labels.contains_key(name) && labels[name].addr.is_some() {
                    panic!("duplicate label");
                }
                labels.entry(name).or_insert_with(LabelInfo::new).addr = Some(code.len());
            }
            Token::LabelRef(name) => {
                code.push(0);
                labels
                    .entry(name)
                    .or_insert_with(LabelInfo::new)
                    .refs
                    .push(code.len() - 1);
            }
            Token::String(s) => generate_string(&mut code, s),
            Token::EscapedString(s) => generate_string(&mut code, s),
            Token::Number(v) => generate_number(&mut code, *v),
            Token::Print => generate_print(&mut code),
            Token::In => code.push(IN),
            Token::Out => code.push(OUT),
            Token::Add => code.push(ADD),
            Token::Sub => code.push(SUB),
            Token::Mul => code.push(MUL),
            Token::Div => code.push(DIV),
            Token::Mod => code.push(MOD),
            Token::Neg => code.push(NEG),
            Token::Inc => code.push(INC),
            Token::Dec => code.push(DEC),
            Token::And => code.push(AND),
            Token::Or => code.push(OR),
            Token::Not => code.push(NOT),
            Token::Xor => code.push(XOR),
            Token::Shl => code.push(SHL),
            Token::Shr => code.push(SHR),
            Token::Pop => code.push(POP),
            Token::Dup => code.push(DUP),
            Token::Swp => code.push(SWP),
            Token::Ovr => code.push(OVR),
            Token::Load => code.push(LOAD),
            Token::Stor => code.push(STOR),
            Token::Jmp => code.push(JMP),
            Token::Je => code.push(JE),
            Token::Jne => code.push(JNE),
            Token::Jg => code.push(JG),
            Token::Jge => code.push(JGE),
            Token::Jl => code.push(JL),
            Token::Jle => code.push(JLE),
            Token::Nop => code.push(NOP),
            Token::Halt => code.push(HALT),
            Token::Rf => code.push(RF),
            Token::Crf => code.push(CRF),
        }
    }

    finalize_labels(&mut code, &mut labels);
    code.push(NOP);

    code
}

fn finalize_labels<'s>(code: &mut Vec<i32>, labels: &mut HashMap<&'s str, LabelInfo>) {
    for (name, info) in labels.iter() {
        if info.addr.is_none() {
            panic!("undefined label: {name}");
        }
        for r in info.refs.iter() {
            code[*r] = info.addr.unwrap() as i32;
        }
    }
}

fn generate_string(code: &mut Vec<i32>, s: &str) {
    code.push(0); // is this correct?
    for c in s.chars().rev() {
        code.push(c as i32);
    }
}

fn generate_number(code: &mut Vec<i32>, v: i32) {
    code.push(v.abs());
    if v < 0 {
        code.push(NEG);
    }
}

fn generate_print(code: &mut Vec<i32>) {
    code.push(RF);
    let prn = code.len();
    let end = prn + 7;
    code.push(DUP);
    code.push(0);
    code.push(end as i32);
    code.push(JE);
    code.push(OUT);
    code.push(prn as i32);
    code.push(JMP);
    code.push(CRF);
    code.push(POP);
}
