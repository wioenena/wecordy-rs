#[macro_export]
macro_rules! define_route {
    ($method: ident, $name: ident, $path: literal) => {
        #[inline]
        pub fn $name() -> $crate::routes::RouteMeta {
            $crate::routes::RouteMeta::new($crate::Method::$method, ::std::borrow::Cow::Borrowed($path))
        }
    };

    ($method: ident, $name: ident, $path: literal, $($arg: ident),+) => {
        #[inline]
        pub fn $name($($arg: impl std::fmt::Display),+) -> $crate::routes::RouteMeta {
            $crate::routes::RouteMeta::new($crate::Method::$method, ::std::borrow::Cow::Owned(format!($path, $($arg),+)))
        }
    };
}
