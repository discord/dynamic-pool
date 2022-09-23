

pub trait DynamicReset {
    fn reset(&mut self);
}

impl<T> DynamicReset for Option<T>
where
    T: DynamicReset,
{
    fn reset(&mut self) {
        if let Some(x) = self {
            x.reset();
        }
    }
}


macro_rules! impl_dynamic_reset {
    () => ();
    ($($name:ident),+; $($idx:tt),+) => (
        #[doc(hidden)]
        impl<$($name: DynamicReset),*> DynamicReset for ($($name,)*) {
            fn reset(&mut self) {
                $(
                    self.$idx.reset();
                )*
            }
        }
    )
}



impl_dynamic_reset!(T0; 0);
impl_dynamic_reset!(T0, T1; 0, 1);
impl_dynamic_reset!(T0, T1, T2; 0, 1, 2);
impl_dynamic_reset!(T0, T1, T2, T3; 0, 1, 2, 3);
impl_dynamic_reset!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5, 6);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6, 6);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_dynamic_reset!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);