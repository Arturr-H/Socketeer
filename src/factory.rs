//! This file is used to make handling functions with variable
//! number of arguments possible (via traits). So it's sort
//! of a function factory.

/* Imports */
use futures_util::Future;

/// All service / endpoint functions will implement
/// this handler for variable amounts of parameters
pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    /// Call the fn with the argv (argument tuple actually)
    fn call(&self, args: Args) -> Self::Future;
}

/// This macro will automatically write implementations
/// for functions with variable number of arguments for 
/// the `Handler<Args>` trait.
macro_rules! write_function_impl ({ $($param:ident)* } => {
        impl<Function, Fut, $($param,)*> Handler<($($param,)*)> for Function 
        where
            Function: Fn($($param,)*) -> Fut,
            Fut: Future 
        {
            #[inline]
            #[allow(non_snake_case)]
            fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
                (self)($($param,)*)
            }
        }
});


/* Method impls */
write_function_impl! { }
write_function_impl! { A }
write_function_impl! { A B }
write_function_impl! { A B C }
write_function_impl! { A B C D }
write_function_impl! { A B C D E }
write_function_impl! { A B C D E F }
write_function_impl! { A B C D E F G }
write_function_impl! { A B C D E F G H }
write_function_impl! { A B C D E F G H I }
write_function_impl! { A B C D E F G H I J }
write_function_impl! { A B C D E F G H I J K }
