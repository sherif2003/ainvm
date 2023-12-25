use std::{str::FromStr, fmt::{Debug, Display}};

use crate::op_code::*;

pub struct Assembler{
    content:Vec<char>,
    i:usize,
    current_instruction:String,
}

impl Assembler{
    pub fn new(content:String)->Self{
        Assembler{
            content:content.chars().collect(),
            i: 0,
            current_instruction: String::from("")
        }
    }

    pub fn assemble(&mut self)->Vec<u8>{
        let mut instructions:Vec<u8>=vec![];

        while self.i<self.content.len() {

            let c=self.content[self.i];
            self.i+=1;

            if !c.is_ascii_whitespace(){
                self.current_instruction+=&c.to_string();
            }

            if
                (
                    c.is_ascii_whitespace()
                    ||
                    self.i==self.content.len()
                )
                &&
                !self.current_instruction.is_empty()
            {
                let mut new_instructions=self.map_current_instruction();
                instructions.append(&mut new_instructions);
                self.current_instruction=String::from("");
            }


        }
        
        return instructions;
    }

    fn map_current_instruction(&mut self)->Vec<u8>{
        let mut instructions:Vec<u8>=vec![];
    
        match self.current_instruction.as_str() {
    
            op_code_name::NOP=>{
                instructions.push(op_code::NOP)
            }
            op_code_name::PRINT=>{
                instructions.push(op_code::PRINT)
            }
            op_code_name::PRINTLN=>{
                instructions.push(op_code::PRINTLN)
            }
            op_code_name::PUSH_I32=>{
                let mut int=self.get_next_int_lit::<i32>(i32::MIN,i32::MAX).to_be_bytes().to_vec();
                let mut int2:[u8;4]=[0;4];
                for i in 0..4{
                    int2[i]=int[i];
                }
                let int3=i32::from_be_bytes(int2);
                println!("{}",int3);
                instructions.push(op_code::PUSH_I32);
                instructions.append(&mut int);
            }
            op_code_name::PUSH_U32=>{
                instructions.push(op_code::PUSH_U32)
            }
            op_code_name::PUSH_F32=>{
                instructions.push(op_code::PUSH_F32)
            }
            op_code_name::PUSH_I64=>{
                instructions.push(op_code::PUSH_I64)
            }
            op_code_name::PUSH_U64=>{
                instructions.push(op_code::PUSH_U64)
            }
            op_code_name::PUSH_F64=>{
                instructions.push(op_code::PUSH_F64)
            }
            op_code_name::PUSH_CHAR=>{
                instructions.push(op_code::PUSH_CHAR)
            }
            op_code_name::PUSH_TRUE=>{
                instructions.push(op_code::PUSH_TRUE)
            }
            op_code_name::PUSH_FALSE=>{
                instructions.push(op_code::PUSH_FALSE)
            }
            op_code_name::POP=>{
                instructions.push(op_code::POP)
            }
            op_code_name::DUP=>{
                instructions.push(op_code::DUP)
            }
            op_code_name::ADD_I32=>{
                instructions.push(op_code::ADD_I32)
            }
            op_code_name::ADD_U32=>{
                instructions.push(op_code::ADD_U32)
            }
            op_code_name::ADD_F32=>{
                instructions.push(op_code::ADD_F32)
            }
            op_code_name::ADD_I64=>{
                instructions.push(op_code::ADD_I64)
            }
            op_code_name::ADD_U64=>{
                instructions.push(op_code::ADD_U64)
            }
            op_code_name::ADD_F64=>{
                instructions.push(op_code::ADD_F64)
            }
            op_code_name::SUB_I32=>{
                instructions.push(op_code::SUB_I32)
            }
            op_code_name::SUB_U32=>{
                instructions.push(op_code::SUB_U32)
            }
            op_code_name::SUB_F32=>{
                instructions.push(op_code::SUB_F32)
            }
            op_code_name::SUB_I64=>{
                instructions.push(op_code::SUB_I64)
            }
            op_code_name::SUB_U64=>{
                instructions.push(op_code::SUB_U64)
            }
            op_code_name::SUB_F64=>{
                instructions.push(op_code::SUB_F64)
            }
            op_code_name::MUL_I32=>{
                instructions.push(op_code::MUL_I32)
            }
            op_code_name::MUL_U32=>{
                instructions.push(op_code::MUL_U32)
            }
            op_code_name::MUL_F32=>{
                instructions.push(op_code::MUL_F32)
            }
            op_code_name::MUL_I64=>{
                instructions.push(op_code::MUL_I64)
            }
            op_code_name::MUL_U64=>{
                instructions.push(op_code::MUL_U64)
            }
            op_code_name::MUL_F64=>{
                instructions.push(op_code::MUL_F64)
            }
            op_code_name::DIV_I32=>{
                instructions.push(op_code::DIV_I32)
            }
            op_code_name::DIV_U32=>{
                instructions.push(op_code::DIV_U32)
            }
            op_code_name::DIV_F32=>{
                instructions.push(op_code::DIV_F32)
            }
            op_code_name::DIV_I64=>{
                instructions.push(op_code::DIV_I64)
            }
            op_code_name::DIV_U64=>{
                instructions.push(op_code::DIV_U64)
            }
            op_code_name::DIV_F64=>{
                instructions.push(op_code::DIV_F64)
            }
            op_code_name::REM_I32=>{
                instructions.push(op_code::REM_I32)
            }
            op_code_name::REM_U32=>{
                instructions.push(op_code::REM_U32)
            }
            op_code_name::REM_F32=>{
                instructions.push(op_code::REM_F32)
            }
            op_code_name::REM_I64=>{
                instructions.push(op_code::REM_I64)
            }
            op_code_name::REM_U64=>{
                instructions.push(op_code::REM_U64)
            }
            op_code_name::REM_F64=>{
                instructions.push(op_code::REM_F64)
            }
            op_code_name::SHR_I32=>{
                instructions.push(op_code::SHR_I32)
            }
            op_code_name::SHR_U32=>{
                instructions.push(op_code::SHR_U32)
            }
            op_code_name::SHR_I64=>{
                instructions.push(op_code::SHR_I64)
            }
            op_code_name::SHR_U64=>{
                instructions.push(op_code::SHR_U64)
            }
            op_code_name::SHL_I32=>{
                instructions.push(op_code::SHL_I32)
            }
            op_code_name::SHL_U32=>{
                instructions.push(op_code::SHL_U32)
            }
            op_code_name::SHL_I64=>{
                instructions.push(op_code::SHL_I64)
            }
            op_code_name::SHL_U64=>{
                instructions.push(op_code::SHL_U64)
            }
            op_code_name::XOR_I32=>{
                instructions.push(op_code::XOR_I32)
            }
            op_code_name::XOR_U32=>{
                instructions.push(op_code::XOR_U32)
            }
            op_code_name::XOR_I64=>{
                instructions.push(op_code::XOR_I64)
            }
            op_code_name::XOR_U64=>{
                instructions.push(op_code::XOR_U64)
            }
            op_code_name::AND_I32=>{
                instructions.push(op_code::AND_I32)
            }
            op_code_name::AND_U32=>{
                instructions.push(op_code::AND_U32)
            }
            op_code_name::AND_I64=>{
                instructions.push(op_code::AND_I64)
            }
            op_code_name::AND_U64=>{
                instructions.push(op_code::AND_U64)
            }
            op_code_name::OR_I32=>{
                instructions.push(op_code::OR_I32)
            }
            op_code_name::OR_U32=>{
                instructions.push(op_code::OR_U32)
            }
            op_code_name::OR_I64=>{
                instructions.push(op_code::OR_I64)
            }
            op_code_name::OR_U64=>{
                instructions.push(op_code::OR_U64)
            }
            op_code_name::NOT_I32=>{
                instructions.push(op_code::NOT_I32)
            }
            op_code_name::NOT_U32=>{
                instructions.push(op_code::NOT_U32)
            }
            op_code_name::NOT_I64=>{
                instructions.push(op_code::NOT_I64)
            }
            op_code_name::NOT_U64=>{
                instructions.push(op_code::NOT_U64)
            }
            op_code_name::NEG_I32=>{
                instructions.push(op_code::NEG_I32)
            }
            op_code_name::NEG_F32=>{
                instructions.push(op_code::NEG_F32)
            }
            op_code_name::NEG_I64=>{
                instructions.push(op_code::NEG_I64)
            }
            op_code_name::NEG_F64=>{
                instructions.push(op_code::NEG_F64)
            }
            op_code_name::I32_TO_U32=>{
                instructions.push(op_code::I32_TO_U32)
            }
            op_code_name::I32_TO_F32=>{
                instructions.push(op_code::I32_TO_F32)
            }
            op_code_name::I32_TO_I64=>{
                instructions.push(op_code::I32_TO_I64)
            }
            op_code_name::I32_TO_U64=>{
                instructions.push(op_code::I32_TO_U64)
            }
            op_code_name::I32_TO_F64=>{
                instructions.push(op_code::I32_TO_F64)
            }
            op_code_name::U32_TO_I32=>{
                instructions.push(op_code::U32_TO_I32)
            }
            op_code_name::U32_TO_F32=>{
                instructions.push(op_code::U32_TO_F32)
            }
            op_code_name::U32_TO_I64=>{
                instructions.push(op_code::U32_TO_I64)
            }
            op_code_name::U32_TO_U64=>{
                instructions.push(op_code::U32_TO_U64)
            }
            op_code_name::U32_TO_F64=>{
                instructions.push(op_code::U32_TO_F64)
            }
            op_code_name::F32_TO_I32=>{
                instructions.push(op_code::F32_TO_I32)
            }
            op_code_name::F32_TO_U32=>{
                instructions.push(op_code::F32_TO_U32)
            }
            op_code_name::F32_TO_I64=>{
                instructions.push(op_code::F32_TO_I64)
            }
            op_code_name::F32_TO_U64=>{
                instructions.push(op_code::F32_TO_U64)
            }
            op_code_name::F32_TO_F64=>{
                instructions.push(op_code::F32_TO_F64)
            }
            op_code_name::I64_TO_I32=>{
                instructions.push(op_code::I64_TO_I32)
            }
            op_code_name::I64_TO_U32=>{
                instructions.push(op_code::I64_TO_U32)
            }
            op_code_name::I64_TO_F32=>{
                instructions.push(op_code::I64_TO_F32)
            }
            op_code_name::I64_TO_U64=>{
                instructions.push(op_code::I64_TO_U64)
            }
            op_code_name::I64_TO_F64=>{
                instructions.push(op_code::I64_TO_F64)
            }
            op_code_name::U64_TO_I32=>{
                instructions.push(op_code::U64_TO_I32)
            }
            op_code_name::U64_TO_U32=>{
                instructions.push(op_code::U64_TO_U32)
            }
            op_code_name::U64_TO_F32=>{
                instructions.push(op_code::U64_TO_F32)
            }
            op_code_name::U64_TO_I64=>{
                instructions.push(op_code::U64_TO_I64)
            }
            op_code_name::U64_TO_F64=>{
                instructions.push(op_code::U64_TO_F64)
            }
            op_code_name::F64_TO_I32=>{
                instructions.push(op_code::F64_TO_I32)
            }
            op_code_name::F64_TO_U32=>{
                instructions.push(op_code::F64_TO_U32)
            }
            op_code_name::F64_TO_F32=>{
                instructions.push(op_code::F64_TO_F32)
            }
            op_code_name::F64_TO_I64=>{
                instructions.push(op_code::F64_TO_I64)
            }
            op_code_name::F64_TO_U64=>{
                instructions.push(op_code::F64_TO_U64)
            }
            _=>{
                panic!("أمر غير متوقع \"{}\"",self.current_instruction)
            }
        }
    
        return instructions;
    }

    fn get_next_int_lit<INT:FromStr+Ord+Display>(&mut self,min:INT,max:INT)->INT
        where INT::Err:Debug{
        
        let mut int_lit;

        if self.content[self.i]=='-'{
            int_lit=String::from("-");
            self.i+=1
        }
        else{
            int_lit=String::from("")
        }

        while self.i<self.content.len(){
            let c=self.content[self.i];
            self.i+=1;
            if c.is_ascii_whitespace(){
                break
            }
            if !c.is_ascii_digit(){
                panic!("يتوقع رقم، وُجِدَ \'{}\'", c)
            }
            int_lit+=&c.to_string();
        }

        let int=int_lit.parse::<INT>();

        match int {
            Ok(val) => return val,
            Err(msg) =>
                panic!("القيمة \'{}\' قد تعدت القيمة المسموح بها من \'{}\' إلى \'{}\'.",int_lit,min,max)
        }
    }
}