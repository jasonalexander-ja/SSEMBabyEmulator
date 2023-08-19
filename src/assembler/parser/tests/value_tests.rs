use super::*;


#[test]
fn test_parse_hex() {
    assert_eq!(Value::parse_hex("A".to_owned()), Ok(Value::Value(10)));
    assert_eq!(Value::parse_hex("-A".to_owned()), Ok(Value::Value(-10)));
    assert_eq!(Value::parse_hex("K".to_owned()), Err(ValueParseError::InvalidHex("K".to_owned())));
}

#[test]
fn test_parse_decimal() {
    assert_eq!(Value::parse_decimal("10".to_owned()), Ok(Value::Value(10)));
    assert_eq!(Value::parse_decimal("-10".to_owned()), Ok(Value::Value(-10)));
    assert_eq!(Value::parse_decimal("K".to_owned()), Err(ValueParseError::InvalidDecimal("K".to_owned())));
}

#[test]
fn test_parse_octal() {
    assert_eq!(Value::parse_octal("12".to_owned()), Ok(Value::Value(10)));
    assert_eq!(Value::parse_octal("-12".to_owned()), Ok(Value::Value(-10)));
    assert_eq!(Value::parse_octal("K".to_owned()), Err(ValueParseError::InvalidOctal("K".to_owned())));
}

#[test]
fn test_parse_binary() {
    assert_eq!(Value::parse_binary("1010".to_owned()), Ok(Value::Value(10)));
    assert_eq!(Value::parse_binary("-1010".to_owned()), Ok(Value::Value(-10)));
    assert_eq!(Value::parse_binary("K".to_owned()), Err(ValueParseError::InvalidBinary("K".to_owned())));
}

#[test]
fn test_parse_tag() {
    assert_eq!(Value::parse_tag_name("tag".to_owned()), Ok(Value::Tag("tag".to_owned())));
    assert_eq!(Value::parse_tag_name("ta g".to_owned()), Err(ValueParseError::InvalidTagName("ta g".to_owned())))
}

#[test]
fn test_parse() {
    assert_eq!(Value::parse("  0xA  "), Ok(Value::Value(10)));
    assert_eq!(Value::parse("  0d10  "), Ok(Value::Value(10)));
    assert_eq!(Value::parse("  0o12  "), Ok(Value::Value(10)));
    assert_eq!(Value::parse("  0b1010  "), Ok(Value::Value(10)));
    assert_eq!(Value::parse("  $foo  "), Ok(Value::Tag("foo".to_owned())));
    assert_eq!(Value::parse("  sadfdsfsda  "), Err(ValueParseError::InvalidValue("sadfdsfsda".to_owned())));
}
