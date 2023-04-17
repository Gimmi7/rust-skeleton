#[macro_export]
macro_rules! wrapped_bigint_impl_from {
    ($wrap:ty, $inner:ty, $($type:ty),+ $(,)?)=>{
        $(
            impl From<$type> for $wrap {
                fn from(x: $type)-> Self{
                    <$inner>::from(x).wrap()
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! wrapped_bigint_impl_ops {
    ($wrap:ty, $($op:ident $func:ident),+ $(,)?)=>{
        $(
            /// owner $op owner
            impl ops::$op for $wrap{
                type Output = $wrap;
                fn $func(self, rhs: Self) -> Self::Output {
                    self.into_inner().$func(rhs.into_inner()).wrap()
                }
            }
            /// ref $op ref
            impl ops::$op for &$wrap{
                type Output = $wrap;
                fn $func(self, rhs: Self) -> Self::Output {
                    self.inner_ref().$func(rhs.inner_ref()).wrap()
                }
            }
            /// owner $op ref
            impl ops::$op<&$wrap> for $wrap{
                type Output = $wrap;
                fn $func(self, rhs: &$wrap) -> Self::Output {
                    self.into_inner().$func(rhs.inner_ref()).wrap()
                }
            }
            /// ref $op owner
            impl ops::$op<$wrap> for &$wrap{
                 type Output = $wrap;
                fn $func(self, rhs: $wrap) -> Self::Output {
                    self.inner_ref().$func(rhs.into_inner()).wrap()
                }
            }
        )+
    }
}