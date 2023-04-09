#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Circle {
    x: u32,
    y: u32,
    radius: u32
}

#[test]
fn test_prototype() {
    let circle_prototype = Circle {
        x: 10,
        y: 10,
        radius: 10
    };
    let mut circle1 = circle_prototype.clone();
    assert_eq!(circle_prototype, circle1);
    circle1.radius = 16;

    println!("{circle_prototype:#?}");
    println!("{circle1:#?}");



}
