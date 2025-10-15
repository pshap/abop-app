// Boilerplate-reduction macros for implementing error conversions

#[macro_export]
macro_rules! impl_error_conversions {
    ($target:ty => {
        $($source:ty => |$param:ident| $conversion:expr),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from($param: $source) -> Self {
                    $conversion
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_bidirectional_conversions {
    ($type_a:ty, $type_b:ty => {
        $type_a_ident:ident => $type_b_ident:ident: |$param_a:ident| $conversion_a:expr,
        $type_b_ident2:ident => $type_a_ident2:ident: |$param_b:ident| $conversion_b:expr $(,)?
    }) => {
        impl From<$type_a> for $type_b {
            fn from($param_a: $type_a) -> Self {
                $conversion_a
            }
        }

        impl From<$type_b> for $type_a {
            fn from($param_b: $type_b) -> Self {
                $conversion_b
            }
        }
    };
}

#[macro_export]
macro_rules! impl_string_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(err.to_string())
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_wrapped_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(err)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_conditional_conversions {
    ($target:ty, $source:ty => {
        $($pattern:pat => $conversion:expr),+ $(,)?
    }) => {
        impl From<$source> for $target {
            fn from(err: $source) -> Self {
                match err {
                    $($pattern => $conversion,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_contextual_conversions {
    ($target:ty => {
        $($source:ty => $variant:ident: $context:expr),+ $(,)?
    }) => {
        $(
            impl From<$source> for $target {
                fn from(err: $source) -> Self {
                    Self::$variant(format!("{}: {}", $context, err))
                }
            }
        )+
    };
}
