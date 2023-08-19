//! # Linker 
//! 
//! This module links tokenised data from the [parser][crate::assembler::parser], 
//! identifying the concrete values of all the symbols, and returning a vector 
//! of all the machine code instructions. 
//! 
//! The main part of this module is [link_parsed_lines][crate::assembler::linker::link_parsed_lines]
//! which takes a vector of [parser::LineType][crate::assembler::parser::LineType]
//! from the parser; identifies all the tag values, and resolves all the tag
//! and instruction refernces to concrete values, returing a vector of 
//! [BabyInstruction][crate::core::instructions::BabyInstruction] representing 
//! machine code instruction. 
//! 
//! This output can used with the [core][crate::core] module to make a runnable 
//! emulation model. 
//! 
//! This will return [LinkingError][crate::assembler::linker_errors::LinkingError] 
//! if a tag reference cannot be bound. 
//! 
//! # Example
//! ```
//! use baby_emulator::assembler::parser;
//! use baby_emulator::assembler::linker;
//! use baby_emulator::assembler::errors::AssemblyError;
//! use baby_emulator::core::instructions::BabyInstruction;
//! 
//! 
//! pub fn assemble(asm: &String) -> Result<Vec<BabyInstruction>, AssemblyError> {
//!     let parse_result = match parser::parse_asm_string(asm, false) {
//!         Ok(v) => v,
//!         Err((l, e)) => return Err(AssemblyError::ParserError(l, e))
//!     };
//!     match linker::link_parsed_lines(parse_result) {
//!         Ok(v) => Ok(v),
//!         Err(e) => Err(AssemblyError::LinkerError(e))
//!     }
//! }
//! ```
//! 

use std::collections::HashMap;
use std::convert::identity;
use crate::core::instructions::BabyInstruction;
use super::parser::{LineType, Value, Instruction};
use super::linker_errors::{LinkingError, TagError};


/// Links the parsed lines into the corresponding machine code. 
/// 
/// Determines the values for all the tags and resolves any refernces 
/// to the tags to the determines value. 
/// 
/// If the all the contained value expressions can be resolved without error it will
/// return an [Ok] with a vector of each instruction [BabyInstruction]. 
/// 
/// Returns a [LinkingError] if an error is encountered resolving the values. 
pub fn link_parsed_lines(lines: Vec<LineType>) -> Result<Vec<BabyInstruction>, LinkingError> {
    let inlined_tags = inline_tags(lines);
    let tag_values = position_tags(&inlined_tags);
    let preprocessed_lines: Vec<UnlinkedData> = inlined_tags.iter()
        .map(|(_, t)| t.clone())
        .collect();
    link_tags(preprocessed_lines, tag_values)
}

/// Takes a list of unlinked values and a collection of tag names and values, 
/// resolves all the tag name references. 
/// 
/// Converts the unlinked data to a tuple of  [BabyInstruction] representing concrete 
/// machine code/program data. 
/// 
/// If the all the contained value expressions can be resolved without error it will
/// return an [Ok] with a vector of each instruction [BabyInstruction]. 
/// 
/// Returns a [LinkingError] if an error is encountered resolving the values. 
/// 
/// # Parameters
/// * `preprocessed_lines` - The unlinked data. 
/// * `tag_values` - The tag names and corresponding values. 
/// 
fn link_tags(preprocessed_lines: Vec<UnlinkedData>, tag_values: HashMap<String, i32>) -> 
    Result<Vec<BabyInstruction>, LinkingError> {
    let mut instructions: Vec<BabyInstruction> = vec![];

    for line in preprocessed_lines {
        let val = match line.resolve(&tag_values) {
            Ok(v) => v,
            Err(e) => return Err(LinkingError::TagError(e))
        };
        instructions.push(val);
    }
    Ok(instructions)
}

/// Takes a vector of tuples of unlinked machine code values and any tag names specified,
/// generates a collection of tag names plus their index in the supplied vector. 
/// 
/// # Example 
/// ```
/// use baby_emulator::assembler::parser::LineType;
/// use baby_emulator::assembler::parser::Value;
/// use baby_emulator::assembler::linker::inline_tags;
/// use baby_emulator::assembler::linker::position_tags;
/// use baby_emulator::assembler::linker::UnlinkedData;
/// 
/// let parsed_lines = vec![
///     LineType::Tag("foo".to_owned()), // tag "foo" at index 0
///     LineType::Absolute(Value::Value(23))
/// ];
/// let inlined = inline_tags(parsed_lines);
/// let tags = position_tags(&inlined);
/// assert_eq!(tags.get("foo"), Some(&0));
/// ```
/// 
pub fn position_tags(lines: &Vec<(Option<String>, UnlinkedData)>) -> HashMap<String, i32> {
    lines.iter().enumerate().filter_map(|(i, (t, _))| match t {
        Some(v) => Some((v.clone(), i as i32)),
        None => None
    }).collect()
}

/// Takes a vector of parsed asm lines, converts them to [UnlinkedData]
/// representing only values to be converted into machine code, with tags 
/// representing positions in the program stack being placed in a tuple 
/// with the correspoding value it references. 
/// 
/// # Parameters 
/// * `lines` - A vector of parsed asm lines. 
/// 
/// # Returns
/// - A vector of tuples of all the unlinked machine code values plus 
/// a `Some(String)` name of a tag if one was specified. 
/// 
/// # Example 
/// ```
/// use baby_emulator::assembler::parser::LineType;
/// use baby_emulator::assembler::parser::Value;
/// use baby_emulator::assembler::linker::inline_tags;
/// use baby_emulator::assembler::linker::UnlinkedData;
/// 
/// let parsed_lines = vec![
///     LineType::Tag("foo".to_owned()),
///     LineType::Absolute(Value::Value(23))
/// ];
/// let inlined = inline_tags(parsed_lines);
/// match &inlined[0] {
///     (Some(tag_name), UnlinkedData::Absolute(Value::Value(line))) => {
///         assert_eq!(tag_name, "foo");
///         assert_eq!(*line, 23);
///     },
///     _ => panic!()
/// }
/// ```
/// 
pub fn inline_tags(lines: Vec<LineType>) -> Vec<(Option<String>, UnlinkedData)> {
    lines.iter().enumerate().map(|(i, v)| {
        let i = if i == 0 { 1 } else { i };
        match &lines.get(i - 1) {
            Some(LineType::Tag(tag)) => (Some(tag.clone()), v.clone()),
            _ => (None::<String>, v.clone())
        }
    })
    .map(|(t, l)| match l {
        LineType::Absolute(v) => Some((t, UnlinkedData::Absolute(v))),
        LineType::Instruction(v) => Some((t, UnlinkedData::Instruction(v))),
        _ => None
    })
    .filter_map(identity)
    .collect()
}

/// Represents either an instruction or an absolute value. 
/// 
/// Both of these are "real" values (I.E. convert to real machine code values),
/// unlike say tags as in [LineType::Tag] received from the parser which are 
/// compile-time calculated values. 
/// 
/// This represents the expression that have not yet had their values fully
/// determined, anc contain say unverified references to tags that need
/// verifiying and resolving to a concrete value. 
#[derive(Clone)]
pub enum UnlinkedData {
    Absolute(Value),
    Instruction(Instruction),
}

impl UnlinkedData {

    /// Accepts a map of tag names and corresponding values and tries 
    /// to resolve the data's concrete value. 
    /// 
    /// If the contained value expression is already a concrete value it will just
    /// return that, if the contained value expression is a tag reference it 
    /// will lookup the tag name in the supplied hashmap and try to find it's value
    /// returning [TagError] if it cannot be found. 
    /// 
    /// # Parameters
    /// 
    /// * `tags` - A hashmap pf tag names and corresponding values. 
    /// 
    pub fn resolve(&self, tags: &HashMap<String, i32>) -> Result<BabyInstruction, TagError> {
        match self {
            UnlinkedData::Absolute(v) => Self::resolve_absolute_value(v, tags),
            UnlinkedData::Instruction(c) => Self::resolve_instruction(c, tags)
        }
    }

    /// Converts an [Instruction] object to [BabyInstruction], resolving the inner 
    /// operand value expression to a concrete value. 
    /// 
    /// If the inner value expression can be determined, it will return [BabyInstruction]. 
    /// 
    /// Will return [TagError] if the value expresion is a tag reference that 
    /// cannot be determined. 
    /// 
    /// # Parameters 
    /// 
    /// * `instr` - The instruction to be resolved. 
    /// * `tags` - A collection of tag names and values to be looked up. 
    /// 
    pub fn resolve_instruction(instr: &Instruction, tags: &HashMap<String, i32>) -> Result<BabyInstruction, TagError> {
        let val = match Self::resolve_value(&instr.get_operand(), tags) {
            Ok(v) => v,
            Err(v) => return Err(TagError::UnknownTagName(v))
        } as u16;
        match instr {
            Instruction::Jump(_) => Ok(BabyInstruction::Jump(val)),
            Instruction::RelativeJump(_) => Ok(BabyInstruction::RelativeJump(val)),
            Instruction::Negate(_) => Ok(BabyInstruction::Negate(val)),
            Instruction::Store(_) => Ok(BabyInstruction::Store(val)),
            Instruction::Subtract(_) => Ok(BabyInstruction::Subtract(val)),
            Instruction::Test => Ok(BabyInstruction::SkipNextIfNegative),
            Instruction::Stop => Ok(BabyInstruction::Stop),
        }
    }

    /// Tries to resolve an absolute value. 
    /// 
    /// Wrapper for [UnlinkedData::resolve_value], returns [BabyInstruction::AbsoluteValue]
    /// if successful, and [TagError::UnknownTagName] if a tag name reference cannot 
    /// be determined. 
    /// 
    /// # Parameters 
    /// * `val` - The value to be resolved. 
    /// * `tags` - A collection of tag names and values to be looked up. 
    /// 
    pub fn resolve_absolute_value(val: &Value, tags: &HashMap<String, i32>) -> Result<BabyInstruction, TagError> {
        match Self::resolve_value(val, tags) {
            Ok(v) => Ok(BabyInstruction::AbsoluteValue(v)),
            Err(v) => Err(TagError::UnknownTagName(v))
        }
    }

    /// Helper function, tries to resolve a value expression. 
    /// 
    /// If the value expression is just [Value::Value] then it will just return
    /// the inner concrete value, if it's a tag reference, it will try to lookup the 
    /// tag value in the supplied hashmap, returning the tag name if it can't be 
    /// found. 
    pub fn resolve_value(val: &Value, tags: &HashMap<String, i32>) -> Result<i32, String> {
        match val {
            Value::Tag(tag) => Self::get_tag(tag, tags),
            Value::Value(v) => return Ok(*v),
        }
    }

    /// Helper function Tries to get a tag's value from a collection 
    /// of tags returns the tag name if it can't be found. 
    pub fn get_tag(tag: &str, tags: &HashMap<String, i32>) -> Result<i32, String> {
        match tags.get(tag).cloned() {
            Some(v) => Ok(v),
            None => Err(tag.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inline_tags() {
        let vec = vec![
            LineType::Tag("foo".to_owned()),
            LineType::Instruction(Instruction::Negate(Value::Tag("foo".to_owned()))),
            LineType::Instruction(Instruction::Negate(Value::Tag("foo".to_owned()))),
        ];
        let res = inline_tags(vec);
        assert_eq!(res.len(), 2);
        match &res[0] {
            (Some(tag), UnlinkedData::Instruction(Instruction::Negate(Value::Tag(tag_ref)))) => {
                assert_eq!(tag, "foo");
                assert_eq!(tag_ref, "foo")
            },
            _ => assert!(false)
        }
        match &res[1] {
            (None, UnlinkedData::Instruction(Instruction::Negate(Value::Tag(tag_ref)))) => 
                assert_eq!(tag_ref, "foo"),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_position_tags() {
        let vec: Vec<(Option<String>, UnlinkedData)> = vec![
            (Some(format!("foo1")), UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
            (None, UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
            (Some(format!("foo2")), UnlinkedData::Instruction(Instruction::Negate(Value::Value(5)))),
        ];

        let tags = position_tags(&vec);
        assert_eq!(tags.iter().len(), 2);
        assert_eq!(tags.get("foo1"), Some(&0));
        assert_eq!(tags.get("foo2"), Some(&2));
    }

    #[test]
    fn test_link_tags_correct() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
        let lines: Vec<UnlinkedData> = vec![
            UnlinkedData::Instruction(Instruction::Jump(Value::Tag("foo".to_owned()))),
            UnlinkedData::Instruction(Instruction::Jump(Value::Value(5))),
        ];
        match link_tags(lines, tags) {
            Ok(res) => {
                assert_eq!(res.len(), 2);
                assert_eq!(res[0], BabyInstruction::Jump(5));
                assert_eq!(res[1], BabyInstruction::Jump(5));
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_link_tags_fail() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
        let lines: Vec<UnlinkedData> = vec![
            UnlinkedData::Instruction(Instruction::Jump(Value::Tag("bar".to_owned()))),
            UnlinkedData::Instruction(Instruction::Jump(Value::Value(5))),
        ];
        match link_tags(lines, tags) {
            Err(e) => {
                assert_eq!(e, LinkingError::TagError(TagError::UnknownTagName("bar".to_owned())))
            },
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn test_link_parsed_lines_correct() {
        let lines: Vec<LineType> = vec![
            LineType::Tag("start".to_owned()),
            LineType::Instruction(Instruction::Negate(Value::Value(2))),
            LineType::Instruction(Instruction::Jump(Value::Tag("start".to_owned()))),
            LineType::Absolute(Value::Value(5)),
        ];
        match link_parsed_lines(lines) {
            Ok(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0], BabyInstruction::Negate(2));
                assert_eq!(v[1], BabyInstruction::Jump(0));
                assert_eq!(v[2], BabyInstruction::AbsoluteValue(5));
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_link_parsed_lines_fail() {
        let lines: Vec<LineType> = vec![
            LineType::Tag("start".to_owned()),
            LineType::Instruction(Instruction::Negate(Value::Value(2))),
            LineType::Instruction(Instruction::Jump(Value::Tag("foo".to_owned()))),
            LineType::Absolute(Value::Value(5)),
        ];
        match link_parsed_lines(lines) {
            Err(e) => {
                assert_eq!(e, LinkingError::TagError(TagError::UnknownTagName("foo".to_owned())))
            },
            Ok(_) => assert!(false),
        }
    }

}

#[cfg(test)]
pub mod unlinked_data_tests {
    
    use super::*;

    #[test]
    fn test_get_tag() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
        match UnlinkedData::get_tag("foo", &tags) {
            Ok(v) => assert_eq!(v, 5),
            Err(_) => assert!(false)
        }
        match UnlinkedData::get_tag("bar", &tags) {
            Err(e) => assert_eq!(e, "bar".to_owned()),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn test_resolve_value() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
        match UnlinkedData::resolve_value(&Value::Value(5), &tags) {
            Ok(v) => assert_eq!(v, 5),
            Err(_) => assert!(false)
        }
        match UnlinkedData::resolve_value(&Value::Tag("foo".to_owned()), &tags) {
            Ok(v) => assert_eq!(v, 5),
            Err(_) => assert!(false)
        }
        match UnlinkedData::resolve_value(&Value::Tag("bar".to_owned()), &tags) {
            Err(e) => assert_eq!(e, "bar".to_owned()),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn test_resolve_absolute_value() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);
        match UnlinkedData::resolve_absolute_value(&Value::Value(5), &tags) {
            Ok(v) => assert_eq!(v, BabyInstruction::AbsoluteValue(5)),
            Err(_) => assert!(false)
        }
        match UnlinkedData::resolve_absolute_value(&Value::Tag("foo".to_owned()), &tags) {
            Ok(v) => assert_eq!(v, BabyInstruction::AbsoluteValue(5)),
            Err(_) => assert!(false)
        }
        match UnlinkedData::resolve_absolute_value(&Value::Tag("bar".to_owned()), &tags) {
            Err(e) => assert_eq!(e, TagError::UnknownTagName("bar".to_owned())),
            Ok(_) => assert!(false)
        }
    }

    fn get_litteral_value_instruction(value: i32, result: u16) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
        vec![
            (Instruction::Jump(Value::Value(value)), Ok(BabyInstruction::Jump(result))),
            (Instruction::RelativeJump(Value::Value(value)), Ok(BabyInstruction::RelativeJump(result))),
            (Instruction::Negate(Value::Value(value)), Ok(BabyInstruction::Negate(result))),
            (Instruction::Store(Value::Value(value)), Ok(BabyInstruction::Store(result))),
            (Instruction::Subtract(Value::Value(value)), Ok(BabyInstruction::Subtract(result))),
            (Instruction::Test, Ok(BabyInstruction::SkipNextIfNegative)),
            (Instruction::Stop, Ok(BabyInstruction::Stop)),
        ]
    }

    fn get_tag_ref_instructions(value: &str, res: u16) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
        vec![
            (Instruction::Jump(Value::Tag(String::from(value))), Ok(BabyInstruction::Jump(res))),
            (Instruction::RelativeJump(Value::Tag(String::from(value))), Ok(BabyInstruction::RelativeJump(res))),
            (Instruction::Negate(Value::Tag(String::from(value))), Ok(BabyInstruction::Negate(res))),
            (Instruction::Store(Value::Tag(String::from(value))), Ok(BabyInstruction::Store(res))),
            (Instruction::Subtract(Value::Tag(String::from(value))), Ok(BabyInstruction::Subtract(res))),
        ]
    }

    fn get_tag_ref_instructions_err(value: &str, err: TagError) -> Vec<(Instruction, Result<BabyInstruction, TagError>)> {
        vec![
            (Instruction::Jump(Value::Tag(String::from(value))), Err(err.clone())),
            (Instruction::RelativeJump(Value::Tag(String::from(value))), Err(err.clone())),
            (Instruction::Negate(Value::Tag(String::from(value))), Err(err.clone())),
            (Instruction::Store(Value::Tag(String::from(value))), Err(err.clone())),
            (Instruction::Subtract(Value::Tag(String::from(value))), Err(err.clone())),
        ]
    }

    #[test]
    fn test_resolve_instruction() {
        let tags: HashMap<String, i32> = HashMap::from([("foo".to_owned(), 5)]);

        get_litteral_value_instruction(5, 5).iter().for_each(|(i, res)| {
            assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
        });

        get_tag_ref_instructions("foo", 5).iter().for_each(|(i, res)| {
            assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
        });

        get_tag_ref_instructions_err("bar", TagError::UnknownTagName("bar".to_owned())).iter().for_each(|(i, res)| {
            assert_eq!(UnlinkedData::resolve_instruction(i, &tags), *res);
        });
    }

}
