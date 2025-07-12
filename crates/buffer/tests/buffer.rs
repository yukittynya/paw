use utils::{Position, Range};
use buffer::buffer::Buffer;

#[test]
fn empty_create() {
    let buffer = Buffer::new();
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.get_line(0).unwrap(), "");
}

#[test]
fn from_text_create() {
    let buffer = Buffer::from_text("hai :3");
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.get_line(0).unwrap(), "hai :3");
}

#[test]
fn from_text_multi_create() {
    let buffer = Buffer::from_text("hai :3\nmrrp");
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.get_line(0).unwrap(), "hai :3");
    assert_eq!(buffer.get_line(1).unwrap(), "mrrp");
}

#[test]
fn single_line_insert() {
    let mut buffer = Buffer::new();
    buffer.insert(Position::new(0, 0), "nya :3").unwrap();
    assert_eq!(buffer.get_line(0).unwrap(), "nya :3");
}

#[test]
fn mutli_line_insert() {
    let mut buffer = Buffer::new();
    buffer.insert(Position::new(0, 0), "Test\nwoof :3").unwrap();
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.get_line(0).unwrap(), "Test");
    assert_eq!(buffer.get_line(1).unwrap(), "woof :3");
}

#[test]
fn delete_inline() {
    let mut buffer = Buffer::from_text("Hello nya :3");
    let res = buffer.delete(Range::new(
        Position::new(0, 1),
        Position::new(0, 4)
    )).unwrap();
    assert_eq!(res, "ello");
}

#[test]
fn delete_multiline() {
    let mut buffer = Buffer::from_text("Hello nya :3\nTesting\nWoah");
    let res = buffer.delete(Range::new(
        Position::new(0, 1),
        Position::new(1, 4)
    )).unwrap();
    assert_eq!(res, "ello nya :3\nTest");
}

#[test]
fn get_text_inline() {
    let buffer = Buffer::from_text("Hai nya :3");
    let res = buffer.get_text(Range::new(
        Position::new(0, 4),
        Position::new(0, 6)
    )).unwrap();
    assert_eq!(res, "nya");
}

#[test]
fn get_text_multiline() {
    let buffer = Buffer::from_text("Hai nya :3\nTesting\nmrrp");
    let res = buffer.get_text(Range::new(
        Position::new(0, 4),
        Position::new(2, 2)
    )).unwrap();
    assert_eq!(res, "nya :3\nTesting\nmr");
}
