use syntax::ast::*;
use syntax_ext::deriving::generic::ty;
use syntax::codemap::{Span, Spanned};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::{ExtParseUtils, ToTokens};
use syntax::ptr::P;
use syntax::print::pprust::ty_to_string;

use syntax::parse::token;

pub fn expand(cx: &mut ExtCtxt, span: Span, item: &Annotatable, push: &mut FnMut(Annotatable),
          trait_name: &str) {
    let trait_name = trait_name.to_string();
    let method_name = trait_name.to_lowercase();
    let method_ident = cx.ident_of(&method_name);
    // Get the that is wrapped by the newtype and do some checks
    let result = match *item {
        Annotatable::Item(ref x) => {
            match x.node {
                ItemKind::Struct(VariantData::Tuple(ref fields, _), _) => {
                    Some((x.ident, cx.ty_ident(span, x.ident), tuple_content(cx, span, x, fields, method_name)))
                },

                //ItemKind::Struct(VariantData::Struct(ref fields, _), _) => {
                //    Some((x.ident, cx.ty_ident(span, x.ident), struct_content(cx, span, x, fields, method_name)))
                //},

                //ItemKind::Enum(ref definition, _) => {
                //    let input_type = x.ident;
                //    Some((x.ident, quote_ty!(cx, Result<$input_type, &'static str>), enum_content(cx, span, x, definition, method_name)))
                //},

                _ => None,
            }
        },
        _ => None,
    };

    let (input_type, output_type, block) = match result {
        Some(x) => x,
        _ => {
            cx.span_fatal(span, &format!("only structs can use `derive({})`", trait_name))
        },
    };


    let t = quote_ty!(cx, T);
    let trait_path = cx.path_all(span, true,
                                 cx.std_path(&["ops", &trait_name]),
                                 vec![],
                                 vec![t],
                                 vec![],
                                 );

    let int = quote_ty!(cx, i32);
    let binding = typebinding_str(cx, span, "Output", int.clone());

    let sub = cx.trait_ref(cx.path_all(span, true,
                              cx.std_path(&["ops", &trait_name]),
                              vec![],
                              vec![int.clone()],
                              vec![binding],
                              ));
    let sub = MyTraitRef(sub);
    //println!("{:#?}", sub);
    let typaram = MyTyParam(typaram_str(cx, span, "T"));


    let code = quote_item!(cx,
        impl<T: ::std::ops::Mul<i32, Output=i32> > $trait_path for $input_type {
            type Output = $output_type;
            fn $method_ident(self, rhs: T) -> $output_type {
                $block
            }
        }
    );
    println!("{:#?}", code);

    // println!("{:#?}", code);

    //push(Annotatable::Item(code));

}

struct MyTraitRef(TraitRef);
struct MyTyParam(TyParam);
struct MyTyParamBound(TyParamBound);

impl ToTokens for MyTyParam {
    fn to_tokens(&self, cx: &ExtCtxt) -> Vec<TokenTree> {
        let s = &self.0;
        let mut v = s.ident.to_tokens(cx);
        // v.push(TokenTree::Token(s.span, token::Colon));
        v

        // TokenTree::Token(s.span, token::Interpolated(token::NtTy(P(s.clone()))))
    }
}

impl ToTokens for MyTraitRef {
    fn to_tokens(&self, cx: &ExtCtxt) -> Vec<TokenTree> {
        let s = &self.0;
        let mut v = cx.ident_of("T").to_tokens(cx);
        v.push(TokenTree::Token(s.path.span, token::Colon));
        v.append(&mut s.path.to_tokens(cx));
        // v.push(TokenTree::Token(s.span, token::Colon));
        println!("{:#?}", v);
        v

        // TokenTree::Token(s.span, token::Interpolated(token::NtTy(P(s.clone()))))
    }
}

impl ToTokens for MyTyParamBound {
    fn to_tokens(&self, cx: &ExtCtxt) -> Vec<TokenTree> {
        let s = &self.0;
        let mut v = vec![];
        match *s {
            TyParamBound::TraitTyParamBound(ref x, _) => {
            },
            _ => panic!(),
        }
        // let mut v = s.ident.to_tokens(cx);
        // v.push(TokenTree::Token(s.span, token::Colon));
        v

        // TokenTree::Token(s.span, token::Interpolated(token::NtTy(P(s.clone()))))
    }
}

fn tuple_content(cx: &mut ExtCtxt, span: Span, item: &P<Item>, fields: &Vec<StructField>, method_name: String) -> P<Expr> {
    let type_name = item.ident;
    let mut exprs: Vec<P<Expr>>= vec![];

    for i in 0..fields.len() {
        let i = &i.to_string();
        exprs.push(cx.parse_expr(format!("rhs.{}(self.{})", method_name, i)));
    }

    cx.expr_call_ident(span, type_name, exprs)
}

fn typaram_str(cx: &mut ExtCtxt, span: Span, name: &str) -> TyParam {
    cx.typaram(span, cx.ident_of(name), P::from_vec(vec![]), None)
}

fn typebinding_str(cx: &mut ExtCtxt, span: Span, name: &str, ty: P<Ty>) -> TypeBinding {
    TypeBinding {
        id: DUMMY_NODE_ID,
        ident: cx.ident_of(name),
        ty: ty,
        span: span,
    }
}
