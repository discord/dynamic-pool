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

impl<T1, T2> DynamicReset for (T1, T2)
where
    T1: DynamicReset,
    T2: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<T1, T2, T3> DynamicReset for (T1, T2, T3)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
    }
}

impl<T1, T2, T3, T4> DynamicReset for (T1, T2, T3, T4)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
    }
}

impl<T1, T2, T3, T4, T5> DynamicReset for (T1, T2, T3, T4, T5)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
    }
}

impl<T1, T2, T3, T4, T5, T6> DynamicReset for (T1, T2, T3, T4, T5, T6)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
    T6: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
        self.5.reset();
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> DynamicReset for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
    T6: DynamicReset,
    T7: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
        self.5.reset();
        self.6.reset();
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> DynamicReset for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
    T6: DynamicReset,
    T7: DynamicReset,
    T8: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
        self.5.reset();
        self.6.reset();
        self.7.reset();
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> DynamicReset for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
    T6: DynamicReset,
    T7: DynamicReset,
    T8: DynamicReset,
    T9: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
        self.5.reset();
        self.6.reset();
        self.7.reset();
        self.8.reset();
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> DynamicReset
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T1: DynamicReset,
    T2: DynamicReset,
    T3: DynamicReset,
    T4: DynamicReset,
    T5: DynamicReset,
    T6: DynamicReset,
    T7: DynamicReset,
    T8: DynamicReset,
    T9: DynamicReset,
    T10: DynamicReset,
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
        self.4.reset();
        self.5.reset();
        self.6.reset();
        self.7.reset();
        self.8.reset();
        self.9.reset();
    }
}
