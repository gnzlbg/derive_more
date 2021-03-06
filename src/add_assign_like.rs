use quote::Tokens;
use syn::{Body, Ident, VariantData, DeriveInput};
use add_like::{tuple_exprs, struct_exprs};
use utils::add_extra_ty_param_bound_simple;

pub fn expand(input: &DeriveInput, trait_name: &str) -> Tokens {
    let trait_ident = Ident::from(trait_name);
    let method_name = trait_name.to_string();
    let method_name = method_name.trim_right_matches("Assign");
    let method_name = method_name.to_lowercase();
    let method_ident = Ident::from(method_name.to_string() + "_assign");
    let input_type = &input.ident;

    let generics = add_extra_ty_param_bound_simple(&input.generics, &trait_ident);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();


    let exprs = match input.body {
        Body::Struct(VariantData::Tuple(ref fields)) => tuple_exprs(fields, &method_ident),
        Body::Struct(VariantData::Struct(ref fields)) => struct_exprs(fields, &method_ident),

        _ => panic!(format!("Only structs can use derive({})", trait_name)),
    };

    quote!(
        impl#impl_generics ::std::ops::#trait_ident for #input_type#ty_generics #where_clause {
            fn #method_ident(&mut self, rhs: #input_type#ty_generics) {
                #(#exprs;
                  )*
            }
        }
    )
}
