use crate::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_trim_rem() {
    assert_eq!(trim("", ';', '\\'), None);
    assert_eq!(trim("     ", ';', '\\'), None);
    assert_eq!(trim(";     ", ';', '\\'), None);
    assert_eq!(trim("     ;", ';', '\\'), None);
    assert_eq!(trim(";;;", ';', '\\'), None);
    assert_eq!(trim(";", ';', '\\'), None);
    assert_eq!(trim("\\;;", ';', '\\'), Some("\\;"));
    assert_eq!(trim(";asdver\\;йцasd     ", ';', '\\'), None);
    assert_eq!(trim(" 日;本語", ';', '\\'), Some("日"));
}
