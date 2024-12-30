pub fn test_type() {
    assert_eq!(1, 1);
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
    assert_eq!(0.1 + 0.2, 0.3);
}