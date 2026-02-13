#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(0 T0);
        $name!(0 T0, 1 T1);
        $name!(0 T0, 1 T1, 2 T2);
        $name!(0 T0, 1 T1, 2 T2, 3 T3);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
        $name!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
    };
}
pub(crate) use all_the_tuples;
