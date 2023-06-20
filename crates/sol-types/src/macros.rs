/// Calls the given macro with all the tuples.
#[rustfmt::skip]
macro_rules! all_the_tuples {
    (@double $name:ident) =>
    {
        $name!((T1 U1));
        $name!((T1 U1), (T2 U2));
        $name!((T1 U1), (T2 U2), (T3 U3));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19), (T20 U20));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19), (T20 U20), (T21 U21));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19), (T20 U20), (T21 U21), (T22 U22));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19), (T20 U20), (T21 U21), (T22 U22), (T23 U23));
        $name!((T1 U1), (T2 U2), (T3 U3), (T4 U4), (T5 U5), (T6 U6), (T7 U7), (T8 U8), (T9 U9), (T10 U10), (T11 U11), (T12 U12), (T13 U13), (T14 U14), (T15 U15), (T16 U16), (T17 U17), (T18 U18), (T19 U19), (T20 U20), (T21 U21), (T22 U22), (T23 U23), (T24 U24));
    };

    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    };
}
