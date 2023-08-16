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
/// return an [Ok] with a vector of tuples of the each instruction [BabyInstruction] 
/// and the corresponding memory address operand [u16]. 
/// 
/// Returns a [LinkingError] if an error is encountered resolving the values. 
pub fn link_parsed_lines(lines: Vec<LineType>) -> Result<Vec<(BabyInstruction, u16)>, LinkingError> {
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
/// machine code/program data and a [u16] representing any memory address operands. 
/// 
/// If the all the contained value expressions can be resolved without error it will
/// return an [Ok] with a vector of tuples of the each instruction [BabyInstruction] 
/// and the corresponding memory address operand [u16]. 
/// 
/// Returns a [LinkingError] if an error is encountered resolving the values. 
/// 
/// # Parameters
/// * `preprocessed_lines` - The unlinked data. 
/// * `tag_values` - The tag names and corresponding values. 
/// 
fn link_tags(preprocessed_lines: Vec<UnlinkedData>, tag_values: HashMap<String, i32>) -> 
    Result<Vec<(BabyInstruction, u16)>, LinkingError> {
    let mut instructions: Vec<(BabyInstruction, u16)> = vec![];

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
pub fn position_tags(lines: &Vec<(Option<String>, UnlinkedData)>) -> HashMap<String, i32> {
    lines.iter().enumerate().map(|(i, (t, _))| match t {
        Some(v) => (v.clone(), i as i32),
        None => ("".to_owned(), 0)
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
/// a [Some(String)] name of a tag if one was specified. 
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
    pub fn resolve(&self, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        match self {
            UnlinkedData::Absolute(v) => Self::resolve_absolute_value(v, tags),
            UnlinkedData::Instruction(c) => Self::resolve_instruction(c, tags)
        }
    }

    /// Converts an [Instruction] object to [BabyInstruction], resolving the inner 
    /// operand value expression to a concrete value. 
    /// 
    /// If the inner value expression can be determined, it will return a tuple of the
    /// corresponding [BabyInstruction] and the operand memory address value. 
    /// 
    /// Will return [TagError] if the value expresion is a tag reference that 
    /// cannot be determined. 
    /// 
    /// # Parameters 
    /// 
    /// * `instr` - The instruction to be resolved. 
    /// * `tags` - A collection of tag names and values to be looked up. 
    /// 
    pub fn resolve_instruction(instr: &Instruction, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        let val = match Self::resolve_value(&instr.get_operand(), tags) {
            Ok(v) => v,
            Err(v) => return Err(TagError::UnknownTagName(v))
        } as u16;
        match instr {
            Instruction::Jump(_) => Ok((BabyInstruction::Jump, val)),
            Instruction::RelativeJump(_) => Ok((BabyInstruction::RelativeJump, val)),
            Instruction::Negate(_) => Ok((BabyInstruction::Negate, val)),
            Instruction::Store(_) => Ok((BabyInstruction::Store, val)),
            Instruction::Subtract(_) => Ok((BabyInstruction::Subtract, val)),
            Instruction::Test => Ok((BabyInstruction::SkipNextIfNegative, 0)),
            Instruction::Stop => Ok((BabyInstruction::Stop, 0)),
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
    pub fn resolve_absolute_value(val: &Value, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        match Self::resolve_value(val, tags) {
            Ok(v) => Ok((BabyInstruction::AbsoluteValue(v), 0)),
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
    pub fn get_tag(tag: &String, tags: &HashMap<String, i32>) -> Result<i32, String> {
        match tags.get(tag).cloned() {
            Some(v) => Ok(v),
            None => Err(tag.clone())
        }
    }
}
