#[macro_export]
macro_rules! _test {
    ( $e:expr ) => {
        println!("{}={:?}", stringify!($e), $e)
    };
}

#[macro_export]
macro_rules! bundle_traits {
    ( $trait_name:ident, $($traits:path),+ ) => {
        trait $trait_name: $($traits +)+ {}
        impl<T: $($traits +)+> $trait_name for T {}
    };

    ( pub $trait_name:ident, $($traits:path),+ ) => {
        pub trait $trait_name: $($traits +)+ {}
        impl<T: $($traits +)+> $trait_name for T {}
    };
}

#[macro_export]
macro_rules! time_it {
    ( $token:tt ) => {{
        let time = std::time::Instant::now();

        let result = $token;

        (result, time.elapsed())
    }};
}

#[macro_export]
macro_rules! module {
    ( $( $module_name:ident ),* ) => {
        $( mod $module_name; )*
    };

    ( pub $( $module_name:ident ),* ) => {
        $( pub mod $module_name; )*
    }
}
