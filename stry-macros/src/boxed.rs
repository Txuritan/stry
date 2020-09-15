// A boxed future proc-macro based off async-trait

use {
    proc_macro2::{Group, Spacing, Span, TokenStream, TokenTree},
    quote::ToTokens,
    std::{iter::FromIterator, mem},
    syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        visit_mut::{self, VisitMut},
        Attribute, Block, ExprPath, ExprStruct, FnArg, GenericArgument, GenericParam, Generics,
        Ident, ImplItem, ItemImpl, Lifetime, Macro, Pat, PatIdent, PatPath, PatStruct,
        PatTupleStruct, Path, PathArguments, QSelf, Receiver, ReturnType, Signature, Stmt, Type,
        TypePath, TypeReference, WhereClause, WherePredicate,
    },
};

#[derive(Copy, Clone)]
pub struct Args {
    pub local: bool,
}

mod kw {
    syn::custom_keyword!(Send);
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        match try_parse(input) {
            Ok(args) if input.is_empty() => Ok(args),
            _ => Err(error()),
        }
    }
}

fn try_parse(input: ParseStream) -> syn::parse::Result<Args> {
    if input.peek(syn::Token![?]) {
        input.parse::<syn::Token![?]>()?;
        input.parse::<kw::Send>()?;

        Ok(Args { local: true })
    } else {
        Ok(Args { local: false })
    }
}

fn error() -> syn::parse::Error {
    let msg = "expected #[box_async] or #[box_async(?Send)]";

    syn::parse::Error::new(Span::call_site(), msg)
}

pub struct Item {
    item: ItemImpl,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();

        if lookahead.peek(syn::Token![impl]) {
            let mut item: ItemImpl = input.parse()?;

            item.attrs = attrs;

            Ok(Item { item })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.item.to_tokens(tokens)
    }
}

pub fn expand(input: &mut Item, is_local: bool) {
    let mut lifetimes = CollectLifetimes::new("'impl");
    lifetimes.visit_type_mut(&mut *input.item.self_ty);
    let params = &input.item.generics.params;
    let elided = lifetimes.elided;
    input.item.generics.params = syn::parse_quote!(#(#elided,)* #params);

    let context = Context {
        generics: &input.item.generics,
        receiver: &input.item.self_ty,
    };

    for inner in &mut input.item.items {
        if let ImplItem::Method(method) = inner {
            let sig = &mut method.sig;

            if sig.asyncness.is_some() {
                let block = &mut method.block;
                let vis = method.vis.clone();
                let has_self = has_self_in_sig(sig) || has_self_in_block(block);

                transform_block(context, sig, block, has_self, is_local);
                transform_signature(context, sig, has_self, is_local);

                method.vis = vis;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Context<'a> {
    generics: &'a Generics,
    receiver: &'a Type,
}

impl Context<'_> {
    fn lifetimes<'a>(&'a self, used: &'a [Lifetime]) -> impl Iterator<Item = &'a GenericParam> {
        self.generics.params.iter().filter(move |param| {
            if let GenericParam::Lifetime(param) = param {
                used.contains(&param.lifetime)
            } else {
                false
            }
        })
    }
}

// Input:
//      async fn f<T>(&self, x: &T) -> Ret;
//
// Output:
//      fn f<'box_async_life_0, 'life1, 'box_async, T>(
//          &'box_async_life_0 self,
//          x: &'box_async_life_1 T,
//      ) -> Pin<Box<dyn Future<Output = Ret> + Send + 'box_async>>
//      where
//          'box_async_life_0: 'box_async,
//          'box_async_life_1: 'box_async,
//          T: 'box_async,
//          Self: Sync + 'box_async;
fn transform_signature(context: Context, sig: &mut Signature, has_self: bool, is_local: bool) {
    sig.fn_token.span = sig.asyncness.take().expect("span").span;

    let ret = match &sig.output {
        ReturnType::Default => quote::quote!(()),
        ReturnType::Type(_, ret) => quote::quote!(#ret),
    };

    // Collect Lifetimes
    let mut lifetimes = CollectLifetimes::new("'box_async_life_");

    for arg in sig.inputs.iter_mut() {
        match arg {
            FnArg::Receiver(arg) => lifetimes.visit_receiver_mut(arg),
            FnArg::Typed(arg) => lifetimes.visit_type_mut(&mut arg.ty),
        }
    }

    // Fix the `where` clause
    let where_clause = sig
        .generics
        .where_clause
        .get_or_insert_with(|| WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::new(),
        });

    for param in sig
        .generics
        .params
        .iter()
        .chain(context.lifetimes(&lifetimes.explicit))
    {
        match param {
            GenericParam::Type(param) => {
                let param = &param.ident;
                where_clause
                    .predicates
                    .push(syn::parse_quote!(#param: 'box_async));
            }
            GenericParam::Lifetime(param) => {
                let param = &param.lifetime;
                where_clause
                    .predicates
                    .push(syn::parse_quote!(#param: 'box_async));
            }
            GenericParam::Const(_) => {}
        }
    }

    for elided in lifetimes.elided {
        sig.generics.params.push(syn::parse_quote!(#elided));
        where_clause
            .predicates
            .push(syn::parse_quote!(#elided: 'box_async));
    }

    sig.generics.params.push(syn::parse_quote!('box_async));

    if has_self {
        where_clause
            .predicates
            .push(syn::parse_quote!(Self: 'box_async));
    }

    // Fix inputs
    for (i, arg) in sig.inputs.iter_mut().enumerate() {
        match arg {
            FnArg::Receiver(Receiver {
                reference: Some(_), ..
            }) => {}
            FnArg::Receiver(arg) => arg.mutability = None,
            FnArg::Typed(arg) => {
                if let Pat::Ident(ident) = &mut *arg.pat {
                    ident.by_ref = None;
                    ident.mutability = None;
                } else {
                    let positional = positional_arg(i);
                    *arg.pat = syn::parse_quote!(#positional);
                }
            }
        }
    }

    let bounds = if is_local {
        quote::quote!('box_async)
    } else {
        quote::quote!(::core::marker::Send + 'box_async)
    };

    sig.output = syn::parse_quote! {
        -> ::core::pin::Pin<Box<
            dyn ::core::future::Future<Output = #ret> + #bounds
        >>
    };
}

// Input:
//     async fn f<T>(&self, x: &T) -> Ret {
//         self + x
//     }
//
// Output:
//     async fn __f<T, AsyncTrait>(_self: &AsyncTrait, x: &T) -> Ret {
//         _self + x
//     }
//     Box::pin(__f::<T, Self>(self, x))
fn transform_block(
    context: Context,
    sig: &mut Signature,
    block: &mut Block,
    has_self: bool,
    _is_local: bool,
) {
    if let Some(Stmt::Item(syn::Item::Verbatim(item))) = block.stmts.first() {
        if block.stmts.len() == 1 && item.to_string() == ";" {
            return;
        }
    }

    let inner = quote::format_ident!("__{}", sig.ident);
    let args = sig.inputs.iter().enumerate().map(|(i, arg)| match arg {
        FnArg::Receiver(Receiver { self_token, .. }) => quote::quote!(#self_token),
        FnArg::Typed(arg) => {
            if let Pat::Ident(PatIdent { ident, .. }) = &*arg.pat {
                quote::quote!(#ident)
            } else {
                positional_arg(i).into_token_stream()
            }
        }
    });

    let mut standalone = sig.clone();
    standalone.ident = inner.clone();

    let generics = context.generics;

    let mut outer_generics = generics.clone();
    for p in &mut outer_generics.params {
        match p {
            GenericParam::Type(t) => t.default = None,
            GenericParam::Const(c) => c.default = None,
            GenericParam::Lifetime(_) => {}
        }
    }

    if !has_self {
        if let Some(mut where_clause) = outer_generics.where_clause {
            where_clause.predicates = where_clause
                .predicates
                .into_iter()
                .filter_map(|mut pred| {
                    if has_self_in_where_predicate(&mut pred) {
                        None
                    } else {
                        Some(pred)
                    }
                })
                .collect();

            outer_generics.where_clause = Some(where_clause);
        }
    }

    let fn_generics = mem::replace(&mut standalone.generics, outer_generics);
    standalone.generics.params.extend(fn_generics.params);
    if let Some(where_clause) = fn_generics.where_clause {
        standalone
            .generics
            .make_where_clause()
            .predicates
            .extend(where_clause.predicates);
    }

    if has_async_lifetime(&mut standalone, block) {
        standalone
            .generics
            .params
            .push(syn::parse_quote!('box_async));
    }

    let types = standalone
        .generics
        .type_params()
        .map(|param| param.ident.clone())
        .collect::<Vec<_>>();

    match standalone.inputs.iter_mut().next() {
        Some(
            arg @ FnArg::Receiver(Receiver {
                reference: Some(_), ..
            }),
        ) => {
            let (lifetime, mutability, self_token) = match arg {
                FnArg::Receiver(Receiver {
                    reference: Some((_, lifetime)),
                    mutability,
                    self_token,
                    ..
                }) => (lifetime, mutability, self_token),
                _ => unreachable!(),
            };
            let under_self = Ident::new("_self", self_token.span);
            let receiver = context.receiver;
            let mut ty = quote::quote!(#receiver);
            if let Type::TraitObject(trait_object) = receiver {
                if trait_object.dyn_token.is_none() {
                    ty = quote::quote!(dyn #ty);
                }
                if trait_object.bounds.len() > 1 {
                    ty = quote::quote!((#ty));
                }
            }
            *arg = syn::parse_quote! {
                #under_self: &#lifetime #mutability #ty
            };
        }
        Some(arg @ FnArg::Receiver(_)) => {
            let (self_token, mutability) = match arg {
                FnArg::Receiver(Receiver {
                    self_token,
                    mutability,
                    ..
                }) => (self_token, mutability),
                _ => unreachable!(),
            };
            let under_self = Ident::new("_self", self_token.span);
            let receiver = context.receiver;
            *arg = syn::parse_quote! {
                #mutability #under_self: #receiver
            };
        }
        Some(FnArg::Typed(arg)) => {
            if let Pat::Ident(arg) = &mut *arg.pat {
                if arg.ident == "self" {
                    arg.ident = Ident::new("_self", arg.ident.span());
                }
            }
        }
        _ => {}
    }

    if let Some(where_clause) = &mut standalone.generics.where_clause {
        // Work around an input bound like `where Self::Output: Send` expanding
        // to `where <AsyncTrait>::Output: Send` which is illegal syntax because
        // `where<T>` is reserved for future use... :(
        where_clause
            .predicates
            .insert(0, syn::parse_quote!((): Sized));
    }

    let mut replace = ReplaceReceiver::with(context.receiver.clone());
    replace.visit_signature_mut(&mut standalone);
    replace.visit_block_mut(block);

    let mut generics = types;
    let consts = standalone
        .generics
        .const_params()
        .map(|param| param.ident.clone());
    generics.extend(consts);

    let allow_non_snake_case = if sig.ident != sig.ident.to_string().to_lowercase() {
        Some(quote::quote!(non_snake_case,))
    } else {
        None
    };

    let brace = block.brace_token;
    let box_pin = quote::quote_spanned!(brace.span=> {
        #[allow(
            #allow_non_snake_case
            unused_parens, // https://github.com/dtolnay/async-trait/issues/118
            clippy::missing_docs_in_private_items,
            clippy::needless_lifetimes,
            clippy::ptr_arg,
            clippy::trivially_copy_pass_by_ref,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding,
        )]
        #standalone #block
        Box::pin(#inner::<#(#generics),*>(#(#args),*))
    });

    *block = syn::parse_quote!(#box_pin);
    block.brace_token = brace;
}

fn positional_arg(i: usize) -> Ident {
    quote::format_ident!("__arg{}", i)
}

fn contains_fn(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|tt| match tt {
        TokenTree::Ident(ident) => ident == "fn",
        TokenTree::Group(group) => contains_fn(group.stream()),
        _ => false,
    })
}

fn prepend_underscore_to_self(ident: &mut Ident) -> bool {
    let modified = ident == "self";

    if modified {
        *ident = Ident::new("_self", ident.span());
    }

    modified
}

struct HasSelf(bool);

impl VisitMut for HasSelf {
    fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
        self.0 |= expr.path.segments[0].ident == "Self";
        visit_mut::visit_expr_path_mut(self, expr);
    }

    fn visit_pat_path_mut(&mut self, pat: &mut PatPath) {
        self.0 |= pat.path.segments[0].ident == "Self";
        visit_mut::visit_pat_path_mut(self, pat);
    }

    fn visit_type_path_mut(&mut self, ty: &mut TypePath) {
        self.0 |= ty.path.segments[0].ident == "Self";
        visit_mut::visit_type_path_mut(self, ty);
    }

    fn visit_receiver_mut(&mut self, _arg: &mut Receiver) {
        self.0 = true;
    }

    fn visit_item_mut(&mut self, _: &mut syn::Item) {
        // Do not recurse into nested items.
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if !contains_fn(mac.tokens.clone()) {
            self.0 |= has_self_in_token_stream(mac.tokens.clone());
        }
    }
}

fn has_async_lifetime(sig: &mut Signature, block: &mut Block) -> bool {
    let mut visitor = HasAsyncLifetime(false);

    visitor.visit_signature_mut(sig);
    visitor.visit_block_mut(block);

    visitor.0
}

struct HasAsyncLifetime(bool);

impl VisitMut for HasAsyncLifetime {
    fn visit_lifetime_mut(&mut self, life: &mut Lifetime) {
        self.0 |= life.to_string() == "'box_async";
    }

    fn visit_item_mut(&mut self, _: &mut syn::Item) {
        // Do not recurse into nested items.
    }
}

struct CollectLifetimes {
    elided: Vec<Lifetime>,
    explicit: Vec<Lifetime>,
    name: &'static str,
}

impl CollectLifetimes {
    fn new(name: &'static str) -> Self {
        CollectLifetimes {
            elided: Vec::new(),
            explicit: Vec::new(),
            name,
        }
    }

    fn visit_opt_lifetime(&mut self, lifetime: &mut Option<Lifetime>) {
        match lifetime {
            None => *lifetime = Some(self.next_lifetime()),
            Some(lifetime) => self.visit_lifetime(lifetime),
        }
    }

    fn visit_lifetime(&mut self, lifetime: &mut Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.next_lifetime();
        } else {
            self.explicit.push(lifetime.clone());
        }
    }

    fn next_lifetime(&mut self) -> Lifetime {
        let name = format!("{}{}", self.name, self.elided.len());
        let life = Lifetime::new(&name, Span::call_site());

        self.elided.push(life.clone());

        life
    }
}

impl VisitMut for CollectLifetimes {
    fn visit_receiver_mut(&mut self, arg: &mut Receiver) {
        if let Some((_, lifetime)) = &mut arg.reference {
            self.visit_opt_lifetime(lifetime);
        }
    }

    fn visit_type_reference_mut(&mut self, ty: &mut TypeReference) {
        self.visit_opt_lifetime(&mut ty.lifetime);

        visit_mut::visit_type_reference_mut(self, ty);
    }

    fn visit_generic_argument_mut(&mut self, gen: &mut GenericArgument) {
        if let GenericArgument::Lifetime(lifetime) = gen {
            self.visit_lifetime(lifetime);
        }

        visit_mut::visit_generic_argument_mut(self, gen);
    }
}

fn has_self_in_sig(sig: &mut Signature) -> bool {
    let mut visitor = HasSelf(false);

    visitor.visit_signature_mut(sig);

    visitor.0
}

fn has_self_in_where_predicate(where_predicate: &mut WherePredicate) -> bool {
    let mut visitor = HasSelf(false);

    visitor.visit_where_predicate_mut(where_predicate);

    visitor.0
}

fn has_self_in_block(block: &mut Block) -> bool {
    let mut visitor = HasSelf(false);

    visitor.visit_block_mut(block);

    visitor.0
}

fn has_self_in_token_stream(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|tt| match tt {
        TokenTree::Ident(ident) => ident == "Self",
        TokenTree::Group(group) => has_self_in_token_stream(group.stream()),
        _ => false,
    })
}

struct ReplaceReceiver {
    with: Type,
}

impl ReplaceReceiver {
    pub fn with(ty: Type) -> Self {
        ReplaceReceiver { with: ty }
    }

    fn self_ty(&self, span: Span) -> Type {
        respan(&self.with, span)
    }

    fn self_to_qself_type(&self, qself: &mut Option<QSelf>, path: &mut Path) {
        let include_as_trait = true;
        self.self_to_qself(qself, path, include_as_trait);
    }

    fn self_to_qself_expr(&self, qself: &mut Option<QSelf>, path: &mut Path) {
        let include_as_trait = false;
        self.self_to_qself(qself, path, include_as_trait);
    }

    fn self_to_qself(&self, qself: &mut Option<QSelf>, path: &mut Path, _include_as_trait: bool) {
        if path.leading_colon.is_some() {
            return;
        }

        let first = &path.segments[0];
        if first.ident != "Self" || !first.arguments.is_empty() {
            return;
        }

        if path.segments.len() == 1 {
            self.self_to_expr_path(path);
            return;
        }

        let span = first.ident.span();
        *qself = Some(QSelf {
            lt_token: syn::Token![<](span),
            ty: Box::new(self.self_ty(span)),
            position: 0,
            as_token: None,
            gt_token: syn::Token![>](span),
        });

        path.leading_colon = Some(
            **path
                .segments
                .pairs()
                .next()
                .expect("leading_colon1")
                .punct()
                .expect("leading_colon2"),
        );

        let segments = mem::replace(&mut path.segments, Punctuated::new());
        path.segments = segments.into_pairs().skip(1).collect();
    }

    fn self_to_expr_path(&self, path: &mut Path) {
        if path.leading_colon.is_some() {
            return;
        }

        let first = &path.segments[0];
        if first.ident != "Self" || !first.arguments.is_empty() {
            return;
        }

        if let Type::Path(self_ty) = self.self_ty(first.ident.span()) {
            let variant = mem::replace(path, self_ty.path);
            for segment in &mut path.segments {
                if let PathArguments::AngleBracketed(bracketed) = &mut segment.arguments {
                    if bracketed.colon2_token.is_none() && !bracketed.args.is_empty() {
                        bracketed.colon2_token = Some(Default::default());
                    }
                }
            }
            if variant.segments.len() > 1 {
                path.segments.push_punct(Default::default());
                path.segments.extend(variant.segments.into_pairs().skip(1));
            }
        } else {
            let span = path.segments[0].ident.span();
            let msg = "Self type of this impl is unsupported in expression position";
            let error = syn::Error::new(span, msg).to_compile_error();
            *path = syn::parse_quote!(::core::marker::PhantomData::<#error>);
        }
    }

    fn visit_token_stream(&self, tokens: &mut TokenStream) -> bool {
        let mut out = Vec::new();
        let mut modified = false;
        let mut iter = tokens.clone().into_iter().peekable();
        while let Some(tt) = iter.next() {
            match tt {
                TokenTree::Ident(mut ident) => {
                    modified |= prepend_underscore_to_self(&mut ident);
                    if ident == "Self" {
                        modified = true;
                        let self_ty = self.self_ty(ident.span());
                        match iter.peek() {
                            Some(TokenTree::Punct(p))
                                if p.as_char() == ':' && p.spacing() == Spacing::Joint =>
                            {
                                let next = iter.next().expect("none");
                                match iter.peek() {
                                    Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                                        let span = ident.span();
                                        out.extend(quote::quote_spanned!(span=> <#self_ty>));
                                    }
                                    _ => out.extend(quote::quote!(#self_ty)),
                                }
                                out.push(next);
                            }
                            _ => out.extend(quote::quote!(#self_ty)),
                        }
                    } else {
                        out.push(TokenTree::Ident(ident));
                    }
                }
                TokenTree::Group(group) => {
                    let mut content = group.stream();
                    modified |= self.visit_token_stream(&mut content);
                    let mut new = Group::new(group.delimiter(), content);
                    new.set_span(group.span());
                    out.push(TokenTree::Group(new));
                }
                other => out.push(other),
            }
        }
        if modified {
            *tokens = TokenStream::from_iter(out);
        }
        modified
    }
}

impl VisitMut for ReplaceReceiver {
    // `Self` -> `Receiver`
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(node) = ty {
            if node.qself.is_none() && node.path.is_ident("Self") {
                *ty = self.self_ty(node.path.segments[0].ident.span());
            } else {
                self.visit_type_path_mut(node);
            }
        } else {
            visit_mut::visit_type_mut(self, ty);
        }
    }

    // `Self::Assoc` -> `<Receiver>::Assoc`
    fn visit_type_path_mut(&mut self, ty: &mut TypePath) {
        if ty.qself.is_none() {
            self.self_to_qself_type(&mut ty.qself, &mut ty.path);
        }
        visit_mut::visit_type_path_mut(self, ty);
    }

    // `Self::method` -> `<Receiver>::method`
    fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
        if expr.qself.is_none() {
            prepend_underscore_to_self(&mut expr.path.segments[0].ident);
            self.self_to_qself_expr(&mut expr.qself, &mut expr.path);
        }
        visit_mut::visit_expr_path_mut(self, expr);
    }

    fn visit_expr_struct_mut(&mut self, expr: &mut ExprStruct) {
        self.self_to_expr_path(&mut expr.path);
        visit_mut::visit_expr_struct_mut(self, expr);
    }

    fn visit_pat_path_mut(&mut self, pat: &mut PatPath) {
        if pat.qself.is_none() {
            self.self_to_qself_expr(&mut pat.qself, &mut pat.path);
        }
        visit_mut::visit_pat_path_mut(self, pat);
    }

    fn visit_pat_struct_mut(&mut self, pat: &mut PatStruct) {
        self.self_to_expr_path(&mut pat.path);
        visit_mut::visit_pat_struct_mut(self, pat);
    }

    fn visit_pat_tuple_struct_mut(&mut self, pat: &mut PatTupleStruct) {
        self.self_to_expr_path(&mut pat.path);
        visit_mut::visit_pat_tuple_struct_mut(self, pat);
    }

    fn visit_item_mut(&mut self, i: &mut syn::Item) {
        match i {
            // Visit `macro_rules!` because locally defined macros can refer to `self`.
            syn::Item::Macro(i) if i.mac.path.is_ident("macro_rules") => {
                self.visit_macro_mut(&mut i.mac)
            }
            // Otherwise, do not recurse into nested items.
            _ => {}
        }
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        // We can't tell in general whether `self` inside a macro invocation
        // refers to the self in the argument list or a different self
        // introduced within the macro. Heuristic: if the macro input contains
        // `fn`, then `self` is more likely to refer to something other than the
        // outer function's self argument.
        if !contains_fn(mac.tokens.clone()) {
            self.visit_token_stream(&mut mac.tokens);
        }
    }
}

fn respan<T>(node: &T, span: Span) -> T
where
    T: ToTokens + Parse,
{
    let tokens = node.to_token_stream();
    let respanned = respan_tokens(tokens, span);

    syn::parse2(respanned).expect("parse2")
}

fn respan_tokens(tokens: TokenStream, span: Span) -> TokenStream {
    tokens
        .into_iter()
        .map(|mut token| {
            token.set_span(span);

            token
        })
        .collect()
}
