
pub fn test_variable() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    let _y = 5;

    let (a,  b): (bool, bool) = (true, false);
    println!("a is {:?}, b is {:?}", a, b)
}