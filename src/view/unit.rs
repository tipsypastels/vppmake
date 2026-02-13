use std::fmt;

pub fn color(color: [u8; 3]) -> impl fmt::Display {
    struct Display([u8; 3]);
    impl fmt::Display for Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "rgb({}, {}, {})", self.0[0], self.0[1], self.0[2])
        }
    }
    Display(color)
}

pub fn absolute(
    [(a, av), (b, bv)]: [(&'static str, u16); 2],
) -> [(&'static str, impl fmt::Display); 2] {
    struct Px(u16);
    impl fmt::Display for Px {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}px", self.0)
        }
    }
    [(a, Px(av)), (b, Px(bv))]
}
