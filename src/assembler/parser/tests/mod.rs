use super::*;


#[cfg(test)]
mod instruction_test;
#[cfg(test)]
mod value_tests;


#[test]
fn test_strip_comments() {
    assert_eq!(strip_comments("jmp 0x3; hello world"), "jmp 0x3".to_owned());
    assert_eq!(strip_comments("jmp 0x3"), "jmp 0x3".to_owned());
}

const EXAMPLE_LINES: &str = 
"
ldn $start_value  ; Loads 10 into the accumulator 
; Commented line
:loop_start
sub $subtract_val ; Subtract 1 from the accumulator 
cmp               ; Skip the next jump instruction if the accumulator is negative 
jmp $loop_start   ; Jump to the start of the loop 
stp               ; Program stops when the accumulator is negative 
    
:subtract_val     ; Value to be subtracted
abs 0d1
 
:start_value      ; Value to start in the accumulator 
abs 0d-10


";

#[test]
fn test_split_filter_lines() {
    assert_eq!(split_filter_lines(&EXAMPLE_LINES.to_owned()).len(), 10)
}

#[test]
fn test_parse_tag() {
    assert_eq!(parse_tag("  test  ".to_owned()), Ok(LineType::Tag("test".to_owned())));
    assert_eq!(parse_tag("test foo".to_owned()), Err(LineParseError::TagError(TagError::TagNameWhitespace("test foo".to_owned()))));
}

fn get_absolute() -> Vec<(String, Result<LineType, LineParseError>)> {
    let v = 10;
    vec![
        ("  0xA  ".to_owned(), Ok(LineType::Absolute(Value::Value(v)))),
        ("  0d10  ".to_owned(), Ok(LineType::Absolute(Value::Value(v)))),
        ("  0o12  ".to_owned(), Ok(LineType::Absolute(Value::Value(v)))),
        ("  0b1010  ".to_owned(), Ok(LineType::Absolute(Value::Value(v)))),
        ("  $tag  ".to_owned(), Ok(LineType::Absolute(Value::Tag("tag".to_owned())))),
        (
            "dasdasdas".to_owned(), 
            Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidValue("dasdasdas".to_owned()))))
        ),
    ]
}

fn get_absolute_err() -> Vec<(String, Result<LineType, LineParseError>)> {
    vec![
        ("0xK".to_owned(), Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidHex("K".to_owned()))))),
        ("0d1J".to_owned(), Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidDecimal("1J".to_owned()))))),
        ("0o9".to_owned(), Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidOctal("9".to_owned()))))),
        ("0b10a10".to_owned(), Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidBinary("10a10".to_owned()))))),
        ("$ta g".to_owned(), Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(ValueParseError::InvalidTagName("ta g".to_owned()))))),
    ]
}

fn get_instructions() -> Vec<(String, Result<LineType, LineParseError>)> {
    vec![
        ("  jmp   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Jump(Value::Value(10))))),
        ("  jrp   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::RelativeJump(Value::Value(10))))),
        ("  ldn   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Negate(Value::Value(10))))),
        ("  sto   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Store(Value::Value(10))))),
        ("  sub   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Subtract(Value::Value(10))))),
        ("  cmp  ".to_owned(), Ok(LineType::Instruction(Instruction::Test))),
        ("  stp  ".to_owned(), Ok(LineType::Instruction(Instruction::Stop))),
        (
            "sadasdasd".to_owned(), 
            Err(LineParseError::InstructionError(InstructionError::UnkownInstruction("sadasdasd".to_owned())))
        ),
    ]
}

fn get_ogn_instructions() -> Vec<(String, Result<LineType, LineParseError>)> {
    vec![
        ("  0xA  , Cl  ".to_owned(), Ok(LineType::Instruction(Instruction::Jump(Value::Value(10))))),
        ("  Add   0xA  , Cl  ".to_owned(), Ok(LineType::Instruction(Instruction::RelativeJump(Value::Value(10))))),
        ("  -  0xA  , C  ".to_owned(), Ok(LineType::Instruction(Instruction::Negate(Value::Value(10))))),
        ("  c, 0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Store(Value::Value(10))))),
        ("  SUB   0xA  ".to_owned(), Ok(LineType::Instruction(Instruction::Subtract(Value::Value(10))))),
        ("  Test  ".to_owned(), Ok(LineType::Instruction(Instruction::Test))),
        ("  Stop  ".to_owned(), Ok(LineType::Instruction(Instruction::Stop))),
        (
            "sadasdasd".to_owned(), 
            Err(LineParseError::InstructionError(InstructionError::UnkownInstruction("sadasdasd".to_owned())))
        ),
    ]
}

pub fn get_lines() -> Vec<(String, Result<LineType, LineParseError>)> {
    let mut res = vec![
        ("   abs   0xA   ".to_owned(), Ok(LineType::Absolute(Value::Value(10)))),
        ("   :foo   ".to_owned(), Ok(LineType::Tag("foo".to_owned()))),
    ];
    res.append(&mut get_instructions());
    res
}

pub fn get_lines_ogn() -> Vec<(String, Result<LineType, LineParseError>)> {
    let mut res = vec![
        ("   abs   0xA   ".to_owned(), Ok(LineType::Absolute(Value::Value(10)))),
        ("   :foo   ".to_owned(), Ok(LineType::Tag("foo".to_owned()))),
    ];
    res.append(&mut get_ogn_instructions());
    res
}

#[test]
fn test_parse_absolute() {
    get_absolute().iter().for_each(|(v, r)| {
        assert_eq!(parse_absolute(v.to_string()), *r);
    });
}

#[test]
fn test_parse_absolute_err() {
    get_absolute_err().iter().for_each(|(v, r)| {
        assert_eq!(parse_absolute(v.to_string()), *r);
    });
}

#[test]
fn test_parse_instruction() {
    get_instructions().iter().for_each(|(v, r)| {
        assert_eq!(parse_instruction(v.to_string()), *r);
    });
}

#[test]
fn test_parse_instruction_ogn() {
    get_ogn_instructions().iter().for_each(|(v, r)| {
        assert_eq!(parse_instruction_ogn(v.to_string()), *r)
    });
}

#[test]
fn test_parse_line() {
    get_lines().iter().for_each(|(v, r)| {
        assert_eq!(parse_line(v, false), *r);
    });
}

#[test]
fn test_parse_line_ogn() {
    get_lines_ogn().iter().for_each(|(v, r)| {
        assert_eq!(parse_line(v, true), *r);
    });
}

#[test]
fn test_parse_lines() {
    let mut correct_lines: Vec<String> = get_instructions().iter()
        .map(|(v, _)| v.clone())
        .collect();
    let incorrect_lines = correct_lines.clone();
    correct_lines.pop();
    match parse_lines(correct_lines.clone(), false) {
        Ok(v) => assert_eq!(v.len(), correct_lines.len()),
        Err(_) => panic!("Failed to parse correct lines. ")
    };
    let err_index = get_instructions().iter()
        .position(|(_, v)| v.is_err())
        .unwrap_or(0);
    match parse_lines(incorrect_lines, false) {
        Err((i, _)) => assert_eq!(i, err_index),
        Ok(_) => panic!("Incorrect lines parsed correctly. ")
    };
}

#[test]
fn test_parse_lines_ogn() {
    let mut correct_lines: Vec<String> = get_ogn_instructions().iter()
        .map(|(v, _)| v.clone())
        .collect();
    let incorrect_lines = correct_lines.clone();
    correct_lines.pop();
    match parse_lines(correct_lines.clone(), true) {
        Ok(v) => assert_eq!(v.len(), correct_lines.len()),
        Err(_) => panic!("Failed to parse correct og notation lines. ")
    };
    let err_index = get_ogn_instructions().iter()
        .position(|(_, v)| v.is_err())
        .unwrap_or(0);
    match parse_lines(incorrect_lines, true) {
        Err((i, _)) => assert_eq!(i, err_index),
        Ok(_) => panic!("Incorrect og notation lines parsed correctly. ")
    };
}
