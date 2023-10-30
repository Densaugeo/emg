fn argument_type_error(node: impl syn::spanned::Spanned,
) -> proc_macro::TokenStream {
  quote::quote_spanned! {
    node.span() => compile_error!("paragen arguments must be `i32`, `i64`, \
      `f32`, or `f64`");
  }.into()
}

fn return_type_error(node: impl syn::spanned::Spanned,
) -> proc_macro::TokenStream {
  quote::quote_spanned! {
    node.span() => compile_error!("paragen return type must be `Result<Scene, \
      i32>`");
  }.into()
}

#[proc_macro_attribute]
pub fn paragen(
  _args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input_fn = syn::parse_macro_input!(input as syn::ItemFn);
  let signature = input_fn.sig.clone();
  let base_name = signature.ident.clone();
  let args = signature.inputs.clone();
  
  let full_name = syn::Ident::new(format!("paragen_{base_name}").as_str(),
    base_name.clone().span());
  
  let mut arg_names: syn::punctuated::Punctuated<syn::Pat, syn::token::Comma> =
    syn::punctuated::Punctuated::new();
  
  let expected_argument_types: Vec<syn::Type> = vec![
    syn::parse_str("i32").unwrap(),
    syn::parse_str("i64").unwrap(),
    syn::parse_str("f32").unwrap(),
    syn::parse_str("f64").unwrap(),
  ];
  
  for pair in args.clone().into_pairs() {
    match pair.into_tuple().0 {
      syn::FnArg::Receiver(receiver) => return argument_type_error(receiver),
      syn::FnArg::Typed(pat_type) => {
        if !expected_argument_types.contains(&pat_type.ty) {
          return argument_type_error(pat_type.ty);
        }
        arg_names.push(*pat_type.pat);
      },
    }
  }
  
  let expected_return_type: syn::Type = syn::parse_str(
    "Result<Scene, ErrorCode>").unwrap();
  
  match signature.output.clone() {
    syn::ReturnType::Type(_, box_type) => {
      if *box_type != expected_return_type {
        return return_type_error(*box_type);
      }
    },
    _ => return return_type_error(signature.clone()),
  }
  
  proc_macro::TokenStream::from(quote::quote! {
    #input_fn
    
    #[automatically_derived]
    #[no_mangle]
    pub extern "C" fn #full_name(#args) -> i32 {
      match paragen::MUTEX_TEST.try_lock() {
        Err(_) => return ErrorCode::Mutex as i32,
        Ok(mut guard) => {
          *guard = Vec::new();
          
          let scene = match #base_name(#arg_names) {
            Err(code) => return code as i32,
            Ok(scene) => scene,
          };
          
          paragen::write_gltf(&mut guard, scene);
        },
      }
      
      ErrorCode::None as i32
    }
  })
}
