// So, the rust code below is broken. You can fix it. You'll need to know basic arithmetic and
// maybe a little bit about how to make conditional statements and expressions in rust. I'll give
// you an example of those.
//
// Here's a conditional that changes your name if it happens to be the wrong name:
//
//      if name == "Florence" {
//          "Flo"
//      } else {
//          name
//      }
//
// This is an example of a conditional expression. The value of the expression can be one thing or
// the other depending on the state of the `name` variable when the expression is evaluated. This
// works equally well with numbers. You'll use this in one case below

/// Returns the area of a shape. The shape's dimensions are given as x and y.
pub fn area(x: u32, y: u32) -> u32 {
    4
}

/// Returns the length of a shape. Here, "length" is defined as the long side.
pub fn length(x: u32, y: u32) -> u32 {
    4
}

#[cfg(test)]
mod geo_tests {
    #[test]
    fn area_2x2() {
        assert!(4 == super::area(2, 2))
    }

    #[test]
    fn area_2x4() {
        assert!(8 == super::area(2, 4), "expected 8, received {}", super::area(2, 4));
    }

    #[test]
    fn area_3x3() {
        assert!(9 == super::area(3, 3));
    }

    #[test]
    fn length_2x2() {
        assert!(2 == super::length(2, 2));
    }

    #[test]
    fn length_2x4() {
        assert!(4 == super::length(4, 2));
    }

    #[test]
    fn length_3x3() {
        assert!(3 == super::length(3, 3));
    }
}
