use crate::Calculator;
use crate::Circle;
use crate::Rectangle;
use crate::Shape;

use rand::Rng;

macro_rules! circumference {
    // if one argument it is circumference for circle
    ($radius:ident) => {
        2.0 * $radius * std::f64::consts::PI
    };
    // if two arguments it is circumference for rectangle
    ($a:ident,$b:ident) => {
        2.0 * $a + 2.0 * $b
    };
}

macro_rules! area {
    // if one argument it is area for circle
    ($radius:ident) => {
        $radius * $radius * std::f64::consts::PI
    };
    // if two arguments it is area for rectangle
    ($a:ident,$b:ident) => {
        $a * $b
    };
}
macro_rules! op {
    ($bound:ident, $a:expr, $b:expr) => {
        $a.$bound($b)
    };
}
#[test]
fn addition() {
    let x_in: i64 = 1;
    let y_in: i64 = 5;
    let mut example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.addition(), op!(checked_add, x_in, y_in));

    let new_x_in: i64 = 8;
    let new_y_in: i64 = 57;
    example.x = new_x_in;
    example.y = new_y_in;

    assert_eq!(example.addition(), op!(checked_add, new_x_in, new_y_in));
}
#[test]
fn subtraction() {
    let x_in: i64 = 1;
    let y_in: i64 = 5;
    let mut example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.subtraction(), op!(checked_sub, x_in, y_in));

    let new_x_in: i64 = 13;
    let new_y_in: i64 = 21;
    example.x = new_x_in;
    example.y = new_y_in;

    assert_eq!(example.subtraction(), op!(checked_sub, new_x_in, new_y_in));
}
#[test]
fn multiplication() {
    let x_in: i64 = 1;
    let y_in: i64 = 5;
    let mut example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.multiplication(), op!(checked_mul, x_in, y_in));

    let new_x_in: i64 = 2;
    let new_y_in: i64 = 473;
    example.x = new_x_in;
    example.y = new_y_in;

    assert_eq!(
        example.multiplication(),
        op!(checked_mul, new_x_in, new_y_in)
    );
}
#[test]
fn division() {
    let x_in: i64 = 1;
    let y_in: i64 = 5;
    let mut example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.division(), op!(checked_div, x_in, y_in));

    let new_x_in: i64 = 458991;
    let new_y_in: i64 = 549;
    example.x = new_x_in;
    example.y = new_y_in;

    assert_eq!(example.division(), op!(checked_div, new_x_in, new_y_in));
}

#[test]
fn overflow_add() {
    let x_in: i64 = i64::MAX;
    let y_in: i64 = 1;
    let example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.addition(), None);
}
#[test]
fn overflow_sub() {
    let x_in: i64 = i64::MIN;
    let y_in: i64 = 1;
    let example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.subtraction(), None);
}
#[test]
fn overflow_mul() {
    let x_in: i64 = i64::MAX / 2 + 1;
    let y_in: i64 = 2;
    let example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.multiplication(), None);
}
#[test]
fn overflow_div() {
    let x_in: i64 = i64::MIN;
    let y_in: i64 = -1;
    let example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.division(), None);
}

#[test]
fn zero_div() {
    let x_in: i64 = 58;
    let y_in: i64 = 0;
    let example = Calculator::new(&x_in, &y_in);

    assert_eq!(example.division(), None);
}

#[test]
fn rectangle_area() {
    let a_in: f64 = 15.0;
    let b_in: f64 = 7.0;
    let rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();

    assert_eq!(rectangle.area(), area!(a_in, b_in));
}
#[test]
fn rectangle_wrong_input() {
    let a_in: f64 = 5.0;
    let b_in: f64 = 7.0;
    let mut rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();

    let new_a_in: f64 = -5.0;
    let res = rectangle.set_a(&new_a_in);

    assert!(res.is_err());
}
#[test]
fn rectangle_area_with_set() {
    let a_in: f64 = 51.0;
    let b_in: f64 = 23.0;
    let mut rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();

    assert_eq!(rectangle.area(), area!(a_in, b_in));

    let new_a_in: f64 = 8.0;
    let res = rectangle.set_a(&new_a_in);
    assert!(res.is_ok());

    assert_eq!(rectangle.area(), area!(new_a_in, b_in));

    let new_b_in: f64 = 5.0;
    let res = rectangle.set_b(&new_b_in);
    assert!(res.is_ok());

    assert_eq!(rectangle.area(), area!(new_a_in, new_b_in));
}
#[test]
fn circle_area() {
    let r_in: f64 = 4.0;
    let circle = Circle::try_new(&r_in).unwrap();

    assert_eq!(circle.area(), area!(r_in)); // an error here will disappear once you implement the area method for Circle
}
#[test]
fn circle_wrong_input() {
    let r_in: f64 = 5.0;
    let mut circle = Circle::try_new(&r_in).unwrap();

    let new_r_in: f64 = -5.0;
    let res = circle.set_r(&new_r_in);

    assert!(res.is_err());
}
#[test]
fn circle_area_with_set() {
    let r_in: f64 = 17.0;
    let mut circle = Circle::try_new(&r_in).unwrap();

    assert_eq!(circle.area(), area!(r_in)); // an error here will disappear once you implement the Shape trait for Circle

    let new_r_in: f64 = 8.0;
    let res = circle.set_r(&new_r_in); // an error here will disappear once you implement the set_r method for Circle
    assert!(res.is_ok());

    assert_eq!(circle.area(), area!(new_r_in)); // an error here will disappear once you implement the Shape trait for Circle
}
#[test]
fn rectangle_circumference() {
    let a_in: f64 = 15.0;
    let b_in: f64 = 7.0;
    let rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();

    assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));
}
#[test]
fn rectangle_circumference_with_set() {
    let a_in: f64 = 584.0;
    let b_in: f64 = 1287.0;
    let mut rectangle = Rectangle::try_new(&a_in, &b_in).unwrap();

    assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));

    let new_a_in: f64 = 8.0;
    let res = rectangle.set_a(&new_a_in);
    assert!(res.is_ok());

    assert_eq!(rectangle.circumference(), circumference!(new_a_in, b_in));

    let new_b_in: f64 = 8.0;
    let res = rectangle.set_b(&new_b_in);
    assert!(res.is_ok());

    assert_eq!(
        rectangle.circumference(),
        circumference!(new_a_in, new_b_in)
    );
}
#[test]
fn circle_circumference() {
    let r_in: f64 = 7.0;
    let circle = Circle::try_new(&r_in).unwrap();

    assert_eq!(circle.circumference(), circumference!(r_in)); // an error here will disappear once you implement the Shape trait for Circle
}
#[test]
fn circle_circumference_with_set() {
    let r_in: f64 = 3.0;
    let mut circle = Circle::try_new(&r_in).unwrap();

    assert_eq!(circle.circumference(), circumference!(r_in)); // an error here will disappear once you implement the Shape trait for Circle

    let new_r_in: f64 = 8.0;
    let res = circle.set_r(&new_r_in); // an error here will disappear once you implement the set_r method for Circle
    assert!(res.is_ok());

    assert_eq!(circle.circumference(), circumference!(new_r_in)); // an error here will disappear once you implement the Shape trait for Circle
}
#[test]
fn random_inputs_calculator() {
    let mut rng = rand::thread_rng();
    for _ in 0..50000 {
        let x_in: i64 = rng.gen();
        let y_in: i64 = rng.gen();
        let example = Calculator::new(&x_in, &y_in);
        assert_eq!(example.addition(), op!(checked_add, x_in, y_in));
        assert_eq!(example.subtraction(), op!(checked_sub, x_in, y_in));
        assert_eq!(example.multiplication(), op!(checked_mul, x_in, y_in));
        assert_eq!(example.division(), op!(checked_div, x_in, y_in));
    }
}
#[test]
fn random_inputs_shapes() {
    let mut rng = rand::thread_rng();
    for _ in 0..50000 {
        let a_in: f64 = rng.gen();
        let b_in: f64 = rng.gen();
        let r_in: f64 = rng.gen();

        let circle = Circle::try_new(&r_in);
        let rectangle = Rectangle::try_new(&a_in, &b_in);

        if r_in < 0.0 {
            assert!(circle.is_err());
        } else {
            let circle = circle.unwrap();
            assert_eq!(circle.circumference(), circumference!(r_in)); // an error here will disappear once you implement the Shape trait for Circle
            assert_eq!(circle.area(), area!(r_in)); // an error here will disappear once you implement the Shape trait for Circle
        }

        if a_in < 0.0 || b_in < 0.0 {
            assert!(rectangle.is_err());
        } else {
            let rectangle = rectangle.unwrap();
            assert_eq!(rectangle.circumference(), circumference!(a_in, b_in));
            assert_eq!(rectangle.area(), area!(a_in, b_in));
        }
    }
}
