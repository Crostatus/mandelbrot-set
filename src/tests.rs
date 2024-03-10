#[cfg(test)]
use super::*;

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20", 'x'), None);
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("1920x1080", 'x'), Some((1920, 1080)));
    assert_eq!(parse_pair::<f32>("0.7x5.5", 'x'), Some((0.7, 5.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("4.20,-0.0525"),
        Some(Complex {
            re: 4.20,
            im: -0.0525
        })
    );
    assert_eq!(parse_complex("4.20,"), None);
}
