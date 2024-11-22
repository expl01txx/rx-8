use color_print::cprintln;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Instructions {
    Mov {
        dest: usize,
        source: usize,
    },
    Add {
        dest: usize,
        source: usize,
    },
    Sub {
        dest: usize,
        source: usize,
    },
    Mult {
        dest: usize,
        source: usize,
    },
    Div {
        dest: usize,
        source: usize,
    },
    Xor {
        dest: usize,
        source: usize,
    },
    Cmp{
        dest: usize,
        source: usize,
    },
    Out{
        source: usize,
    },
    In{
        dest: usize,
    },
    Hlt,
}

macro_rules! register_check {
    ($arg:ident, $id:expr) => {
        if $arg.is_none(){
            return Err(format!("Incorrect #{} argument", $id).to_string());
        }
        let $arg = $arg.unwrap().trim();
        if $arg.chars().nth(0).unwrap() != 'r'{
            return Err(format!("Argument #{} must be a register", $id).to_string());
        }
    };
}

macro_rules! value_check {
    ($arg:ident, $id:expr) => {
        if $arg.is_none(){
            return Err(format!("Incorrect #{} argument", $id).to_string());
        }
        let $arg = $arg.unwrap();
        if $arg.chars().nth(0).unwrap() == 'r'{
            return Err(format!("Argument #{} must be a number", $id).to_string());
        }
    };
}

macro_rules! get_reg {
    ($self:ident, $args:ident, $name:ident, $id:expr) => {
        let $name = $args.next();
        register_check!($name, $id);
        let $name = $self.parse_register($name).unwrap();
    };
}

pub struct Parser {
    instructions: Vec<Instructions>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            instructions: Vec::new(),
        }
    }

    fn parse_register(&mut self, line: &str) -> Result<usize, std::num::ParseIntError> {
        line[1..].parse::<usize>()
    }

    fn parse_num(&mut self, line: &str) -> Result<usize, std::num::ParseIntError> {
        line.parse::<usize>()
    }

    fn parse_line(&mut self, line: &str) -> Result<Instructions, String> {
        if line == "hlt" {
            return Ok(Instructions::Hlt);
        }

        let mut parts = line.split_once(" ");

        if parts.is_none() {
            return Err("Failed to parse operation tag".to_owned());
        }

        let parts = parts.unwrap();
        let operation = parts.0;
        let args = parts.1;
        let mut args = args.split(",");

        //Generate instuctions
        if operation == "mov" {
            get_reg!(self, args, reg, 1);

            let value = args.next();
            value_check!(value, 2);
            let value = self.parse_num(value.trim()).unwrap();

            return Ok(Instructions::Mov { dest: reg, source: value });
        } else if operation == "add" {
            get_reg!(self, args, reg1, 1);
            get_reg!(self, args, reg2, 2);
            return Ok(Instructions::Add { dest: reg1, source: reg2 });
        } else if operation == "sub" {
            get_reg!(self, args, reg1, 1);
            get_reg!(self, args, reg2, 2);
            return Ok(Instructions::Sub { dest: reg1, source: reg2 });
        } else if operation == "mult" {
            get_reg!(self, args, reg1, 1);
            get_reg!(self, args, reg2, 2);
            return Ok(Instructions::Mult { dest: reg1, source: reg2 });
        } else if operation == "div" {
            get_reg!(self, args, reg1, 1);
            get_reg!(self, args, reg2, 2);
            return Ok(Instructions::Div { dest: reg1, source: reg2 });
        } else if operation == "xor" {
            get_reg!(self, args, reg1, 1);
            get_reg!(self, args, reg2, 2);
            return Ok(Instructions::Xor { dest: reg1, source: reg2 });
        } else if operation == "cmp" {
                get_reg!(self, args, reg1, 1);
                get_reg!(self, args, reg2, 2);
                return Ok(Instructions::Cmp { dest: reg1, source: reg2 });
        } else if operation == "out" {
            get_reg!(self, args, reg, 1);
            return Ok(Instructions::Out { source: reg });
        } else if operation == "in" {
            get_reg!(self, args, reg, 1);
            return Ok(Instructions::In { dest: reg });
        }

        return Err(format!("Unknown operand, {}", operation).to_owned());
    }

    pub fn parse(&mut self, src: &str, filename: &str) -> Result<&Vec<Instructions>, ()> {
        let lines = src.split('\n');
        let mut n = 1;
        for line in lines {
            let line = line.trim();

            if line == "" || line.chars().nth(0).unwrap() == '#' {
                continue;
            }

            let inst = self.parse_line(line.to_lowercase().as_str());
            if inst.is_err() {
                cprintln!("<red>[{filename}:{n}]: {}<red>", inst.err().unwrap());
                return Err(());
            }

            self.instructions.push(inst.unwrap());

            n += 1;
        }
        return Ok(&self.instructions);
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn parser_test1(){
        let mut parser = Parser::new();
        parser.parse("", "").unwrap();
    }

    #[test]
    fn parser_test2(){
        let src = r#"
        mov r0, 0
        add r0,     r1
        sub     r0, r2
        mult r2,    r3
        xor r3,   r0
        cmp  r0, r1
        out r0
        in    r1
        hlt  
        "#;
        let mut parser = Parser::new();
        parser.parse(src, "").unwrap();
    }

}