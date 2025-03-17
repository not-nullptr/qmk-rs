mod components;
mod hello_world;
mod home;
mod rgb;
mod settings;
mod transition;

pub use hello_world::*;
pub use home::*;
pub use rgb::*;
pub use settings::*;
pub use transition::*;

#[macro_export]
macro_rules! define_options {
    ($($name:expr => $fun:expr; $fn_ident:ident),*) => {
        #[allow(dead_code)]
        const OPTION_TEXT: &[&str] = &[$($name),*];
        type CreatePage = fn() -> ::alloc::boxed::Box<dyn crate::page::Page>;
        $(
            fn $fn_ident() -> ::alloc::boxed::Box<dyn crate::page::Page> {
                ::alloc::boxed::Box::new($fun())
            }
        )*

        #[allow(dead_code)]
        const OPTION_CONSTRUCTORS: &[CreatePage] = &[$($fn_ident),*];
    };
}
