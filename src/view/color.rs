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
