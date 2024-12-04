use quote::quote;

pub fn create_panic_on_use() -> proc_macro2::TokenStream {
    quote! {
      #[derive(Debug)]
      struct PanicOnUse<T>(T);

      impl<T> PanicOnUse<T> {
          fn new(value: T) -> Self {
              Self(value)
          }
      }

      impl<T> std::ops::Deref for PanicOnUse<T> {
          type Target = T;
          fn deref(&self) -> &Self::Target {
              panic!("🔥 YOU TRIED TO ACCESS A MUST-NOT-USE VALUE! 🔥")
          }
      }

      impl<T> std::ops::DerefMut for PanicOnUse<T> {
          fn deref_mut(&mut self) -> &mut Self::Target {
              panic!("🔥 YOU TRIED TO MUTABLY ACCESS A MUST-NOT-USE VALUE! 🔥")
          }
      }

      impl<T> std::fmt::Display for PanicOnUse<T> where T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::ops::Deref;

            let inner: &T = self.deref();
            write!(f, "{}", inner)
        }
      }
    }
}
