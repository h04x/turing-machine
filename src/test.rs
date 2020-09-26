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
fn test_parse_rule_line() {
    //"",
    //panic!()
}
