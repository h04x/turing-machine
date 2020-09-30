use crate::TapeMotion;
use crate::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_trim_rem() {
    assert_eq!(trim(r#""#), None);
    assert_eq!(trim(r#" "#), None);
    assert_eq!(trim(r#";"#), None);
    assert_eq!(trim(r#"\"#), Some(r#"\"#));
    assert_eq!(trim(r#"a"#), Some(r#"a"#));
    assert_eq!(trim(r#";;"#), None);
    assert_eq!(trim(r#"; "#), None);
    assert_eq!(trim(r#" ;"#), None);
    assert_eq!(trim(r#";\"#), None);
    assert_eq!(trim(r#"\;"#), Some(r#"\;"#));
    assert_eq!(trim(r#"\\"#), Some(r#"\\"#));
    assert_eq!(trim(r#"\a"#), Some(r#"\a"#));
    assert_eq!(trim(r#"a\"#), Some(r#"a\"#));
    assert_eq!(trim(r#"aa"#), Some(r#"aa"#));
    assert_eq!(trim(r#"a "#), Some(r#"a"#));
    assert_eq!(trim(r#" a"#), Some(r#"a"#));
    assert_eq!(trim(r#" \"#), Some(r#"\"#));
    assert_eq!(trim(r#"\ "#), Some(r#"\"#));
    assert_eq!(trim(r#";;;"#), None);
    assert_eq!(trim(r#";\;"#), None);
    assert_eq!(trim(r#";\\"#), None);
    assert_eq!(trim(r#"\a;"#), Some(r#"\a"#));
    assert_eq!(trim(r#"\;\"#), Some(r#"\;\"#));
    assert_eq!(trim(r#"\\;"#), Some(r#"\\"#));
    assert_eq!(trim(r#"\;;"#), Some(r#"\;"#));
    assert_eq!(trim(r#"     "#), None);
    assert_eq!(trim(r#";     "#), None);
    assert_eq!(trim(r#"     ;"#), None);
    assert_eq!(trim(r#"\\;zxc"#), Some(r#"\\"#));
    assert_eq!(trim(r#";asdver\\;йцasd     "#), None);
    assert_eq!(trim(r#" 日;本語"#), Some("日"));
}

#[test]
fn test_split_arrow() {
    assert_eq!(split_arrow(""), Err(()));
    assert_eq!(split_arrow(r#"\"#), Err(()));
    assert_eq!(split_arrow("-"), Err(()));
    assert_eq!(split_arrow(">"), Err(()));
    assert_eq!(split_arrow("a"), Err(()));
    assert_eq!(split_arrow("ab"), Err(()));
    assert_eq!(split_arrow(r#"\b"#), Err(()));
    assert_eq!(split_arrow(r#"\\b"#), Err(()));
    assert_eq!(split_arrow("->"), Ok(("", "")));
    assert_eq!(split_arrow(r#"\->"#), Err(()));
    assert_eq!(split_arrow(r#"-\>"#), Err(()));
    assert_eq!(split_arrow(r#"a->b"#), Ok(("a", "b")));
    assert_eq!(split_arrow(r#"abc->def\"#), Ok(("abc", r#"def\"#)));
    assert_eq!(split_arrow(r#"abc->de->f"#), Err(()));
    assert_eq!(split_arrow(r#"abc->de\->f"#), Err(()));
    assert_eq!(split_arrow(r#"abc->de-\>f"#), Err(()));
    assert_eq!(split_arrow(r#"abc->de\-\>f"#), Ok(("abc", r#"de\-\>f"#)));
    assert_eq!(split_arrow(r#"abc->"#), Ok(("abc", r#""#)));
    assert_eq!(split_arrow(r#"->def"#), Ok(("", r#"def"#)));
    assert_eq!(split_arrow(r#"日本->語"#), Ok(("日本", r#"語"#)));
    assert_eq!(split_arrow(r#"日本\->語"#), Err(()));
    assert_eq!(split_arrow(r#"日本\\->語"#), Ok((r#"日本\\"#, r#"語"#)));
}

#[test]
fn test_parse_left() {
    assert_eq!(parse_left(""), Err(()));
    assert_eq!(parse_left("a"), Err(()));
    assert_eq!(parse_left(r#"\"#), Err(()));
    assert_eq!(parse_left(r#"\a"#), Err(()));
    assert_eq!(parse_left("ab"), Ok(("a".to_string(), 'b')));
    assert_eq!(parse_left(r#"\\"#), Err(()));
    assert_eq!(parse_left(r#"\ab"#), Ok(("a".to_string(), 'b')));
    assert_eq!(parse_left(r#"\\\"#), Err(()));
    assert_eq!(parse_left(r#"a\b"#), Ok(("a".to_string(), 'b')));
    assert_eq!(parse_left(r#"a\\b"#), Ok((r#"a\"#.to_string(), 'b')));
    assert_eq!(parse_left(r#"a\\b\"#), Err(()));
    assert_eq!(parse_left(r#"\\\\"#), Ok((r#"\"#.to_string(), '\\')));
    assert_eq!(parse_left(r#"\\ab"#), Ok((r#"\a"#.to_string(), 'b')));
    assert_eq!(parse_left(r#"ab\\"#), Ok((r#"ab"#.to_string(), '\\')));
    assert_eq!(parse_left(r#"abcdef"#), Ok((r#"abcde"#.to_string(), 'f')));
    assert_eq!(
        parse_left(r#"a日c本語"#),
        Ok((r#"a日c本"#.to_string(), '語'))
    );
    assert_eq!(parse_left(r#"abcde\\\"#), Err(()));
    assert_eq!(
        parse_left(r#"abcde\\\\"#),
        Ok((r#"abcde\"#.to_string(), '\\'))
    );
    assert_eq!(
        parse_left(r#"abcde\\\x"#),
        Ok((r#"abcde\"#.to_string(), 'x'))
    );
}

#[test]
fn test_parse_right() {
    assert_eq!(parse_right(""), Err(()));
    assert_eq!(parse_right("aR"), Err(()));
    assert_eq!(
        parse_right("abR"),
        Ok((r#"a"#.to_string(), 'b', TapeMotion::Right))
    );
    assert_eq!(parse_right("abcD"), Err(()));
    assert_eq!(
        parse_right(r#"ab\L"#),
        Ok((r#"a"#.to_string(), 'b', TapeMotion::Left))
    );
    assert_eq!(parse_right(r#"abR\"#), Err(()));
    assert_eq!(
        parse_right(r#"a\bN"#),
        Ok((r#"a"#.to_string(), 'b', TapeMotion::None))
    );
    assert_eq!(
        parse_right(r#"ab\\R"#),
        Ok((r#"ab"#.to_string(), '\\', TapeMotion::Right))
    );
    assert_eq!(
        parse_right(r#"日本\語R"#),
        Ok((r#"日本"#.to_string(), '語', TapeMotion::Right))
    );
    assert_eq!(
        parse_right(r#"\\\\\R"#),
        Ok((r#"\"#.to_string(), '\\', TapeMotion::Right))
    );
    assert_eq!(
        parse_right(r#"\a\b\c\\N"#),
        Ok((r#"abc"#.to_string(), '\\', TapeMotion::None))
    );
}
