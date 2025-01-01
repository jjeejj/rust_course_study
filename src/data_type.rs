pub fn test_type() {
    assert_eq!(1, 1);
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
    // assert_eq!(0.1 + 0.2, 0.3);

    let c: f32 = (-42.0_f32).sqrt();
    if c.is_nan() {
        println!("c is NaN");
        return;
    }
    assert_eq!(c,c);
}