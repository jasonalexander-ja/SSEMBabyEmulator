use std::collections::HashMap;
use std::convert::identity;
use crate::core::{MEMORY_WORDS, instructions::BabyInstruction};
use super::parser::{LineType, Value, Command};
use super::linker_errors::{LinkingError, TagError};


pub fn link_parsed_lines(lines: Vec<LineType>) -> Result<[i32; MEMORY_WORDS], LinkingError> {
    let inlined_tags = inline_tags(lines);
    let tag_values = position_tags(&inlined_tags);
    let preprocessed_lines: Vec<TaglessLine> = inlined_tags.iter()
        .map(|(_, t)| t.clone())
        .collect();
    link_tags(preprocessed_lines, tag_values)
}

fn link_tags(preprocessed_lines: Vec<TaglessLine>, tag_values: HashMap<String, i32>) -> Result<[i32; MEMORY_WORDS], LinkingError> {
    let mut instructions: Vec<(BabyInstruction, u16)> = vec![];

    for line in preprocessed_lines {
        let val = match line.resolve(&tag_values) {
            Ok(v) => v,
            Err(e) => return Err(LinkingError::TagError(e))
        };
        instructions.push(val);
    }
    Ok(BabyInstruction::to_numbers(instructions))
}

fn position_tags(lines: &Vec<(Option<String>, TaglessLine)>) -> HashMap<String, i32> {
    lines.iter().enumerate().map(|(i, (t, _))| match t {
        Some(v) => (v.clone(), i as i32),
        None => ("".to_owned(), 0)
    }).collect()
}

pub fn inline_tags(lines: Vec<LineType>) -> Vec<(Option<String>, TaglessLine)> {
    lines.iter().enumerate().map(|(i, v)| {
        let i = if i == 0 { 1 } else { i };
        match &lines.get(i - 1) {
            Some(LineType::Tag(tag)) => (Some(tag.clone()), v.clone()),
            _ => (None::<String>, v.clone())
        }
    })
    .map(|(t, l)| match l {
        LineType::Absolute(v) => Some((t, TaglessLine::Absolute(v))),
        LineType::Command(v) => Some((t, TaglessLine::Command(v))),
        _ => None
    })
    .filter_map(identity)
    .collect()
}

#[derive(Clone)]
pub enum TaglessLine {
    Absolute(Value),
    Command(Command),
}

impl TaglessLine {
    pub fn resolve(&self, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        match self {
            TaglessLine::Absolute(v) => Self::resolve_absolute_value(v, tags),
            TaglessLine::Command(c) => Self::resolve_command(c, tags)
        }
    }

    fn resolve_command(comm: &Command, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        let val = match Self::resolve_value(&comm.get_operand(), tags) {
            Ok(v) => v,
            Err(v) => return Err(TagError::UnknownTagName(v))
        } as u16;
        match comm {
            Command::Jump(_) => Ok((BabyInstruction::Jump, val)),
            Command::RelativeJump(_) => Ok((BabyInstruction::RelativeJump, val)),
            Command::Negate(_) => Ok((BabyInstruction::Negate, val)),
            Command::Store(_) => Ok((BabyInstruction::Store, val)),
            Command::Subtract(_) => Ok((BabyInstruction::Subtract, val)),
            Command::Test => Ok((BabyInstruction::SkipNextIfNegative, 0)),
            Command::Stop => Ok((BabyInstruction::Stop, 0)),
        }
    }

    fn resolve_absolute_value(val: &Value, tags: &HashMap<String, i32>) -> Result<(BabyInstruction, u16), TagError> {
        match Self::resolve_value(val, tags) {
            Ok(v) => Ok((BabyInstruction::AbsoluteValue(v), 0)),
            Err(v) => Err(TagError::UnknownTagName(v))
        }
    }

    fn resolve_value(val: &Value, tags: &HashMap<String, i32>) -> Result<i32, String> {
        match val {
            Value::Tag(tag) => match tags.get(tag).cloned() {
                Some(v) => Ok(v),
                None => Err(tag.clone())
            },
            Value::Value(v) => return Ok(*v)
        }
    }
}
