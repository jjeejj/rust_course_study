
pub fn test_variable() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    let _y = 5;

    // 解构表达式
    let (a,  b): (bool, bool) = (true, false);
    println!("a is {:?}, b is {:?}", a, b);

    struct Struct {
        e: i32
    }
    let (a, b, c,d,e);
    (a,b) = (1,2);
    (c, .., d, _) = (1,2,3,4,5);
    Struct { e, .. } = Struct { e: 1 };
}