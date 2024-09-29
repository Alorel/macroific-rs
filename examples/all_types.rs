//! We'll build a struct that contains every [`syn`] type as an argument, then parse a big
//! [`Attribute`].

fn main() {
    println!("Run me with `cargo test --features full,attr_parse --example all_types`");
}

#[cfg(all(test, feature = "full", feature = "attr_parse"))]
mod test {
    use syn::punctuated::Punctuated;

    use {macroific::prelude::*, proc_macro2::*, syn::*};

    /// Helper macro to define the struct, so we don't have to repeat ourselves too much
    macro_rules! define {
        ($struct: ident => $($ty: ident),+) => {
            #[derive(AttributeOptions, ParseOption)]
            #[allow(non_snake_case)]
            #[deny(dead_code)]
            struct $struct {
                $(
                    #[attr_opts(default(false))]
                    $ty: $ty,
                )+
            }
        };
    }

    define!(AllTypes =>
        bool, String, char, f32, f64, u8, i8, u16, i16, u32, i32, u64, i64, usize, isize,
        Expr, AngleBracketedGenericArguments, ConstParam, Abi, BareFnArg, Ident, Path, Meta, MetaList, MetaNameValue, Visibility,
        Lifetime, LifetimeParam, BoundLifetimes, TypeParamBound, TraitBound, TypeParam, GenericParam, WherePredicate,
        Lit, LitBool, LitByteStr, LitByte, LitStr, LitChar, LitInt, LitFloat, Literal,
        Type, TypeArray, TypeBareFn, TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple,
        ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField, ExprForLoop, ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprUnary, ExprUnsafe, ExprWhile, ExprYield
    );

    // Todo once I figure out what they look like:
    // TypeGroup, TypeImplTrait, TypeMacro, TypeNever, TypeParen, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple,

    #[test]
    fn parse_every_type() {
        let allty = AllTypes::from_attr(parse_quote! { #[whatever_your_macro_attr_is(
            bool,
            String("hello"),
            char('c'),
            f32(4.2),
            f64(6.9),
            u8(1),
            i8(-1),
            u16(2),
            i16(-2),
            u32(3),
            i32(-3),
            u64(4),
            i64(-4),
            usize(5),
            isize(-5),
            Expr("lit expr"),
            AngleBracketedGenericArguments(<String>),
            ConstParam(const FOO: usize),
            Abi(extern "Foo"),
            BareFnArg(&mut str),
            Ident(foo_ident),
            Path(::core::Foo),
            Meta(derive(Copy)),
            MetaList(derive(Copy, Clone)),
            MetaNameValue(foo = Bar),
            Visibility(pub(crate)),
            Lifetime('foo),
            LifetimeParam('foo: 'bar + 'qux),
            BoundLifetimes(for<'x>),
            TraitBound(?Sized),
            TypeParamBound(?Sized),
            TypeParam(T: Clone),
            GenericParam(T: Eq + PartialEq),
            WherePredicate(T: Clone + Copy),
            Lit(true),
            LitBool = false,
            LitByteStr(b"foo"),
            LitStr("foo"),
            LitByte(b'@'),
            LitChar('_'),
            LitInt(42),
            LitFloat(4.2),
            Literal("Litter"),
            Type(bool),
            TypeArray([u8; 4]),
            TypeBareFn(fn(&str, String) -> ::std::io::Result<()>),
            TypeImplTrait(impl Foo + Bar),
            TypeInfer(_),
            TypeMacro(vec![]),
            TypeNever(!),
            TypeParen((u8)),
            TypePath(foo::Bar),
            TypePtr(*const u8),
            TypeReference(&u8),
            TypeSlice([u8]),
            TypeTraitObject(dyn Foo + Bar),
            TypeTuple((u8, u8)),
            ExprArray([1, 2, 3]),
            ExprAssign(a = b),
            ExprAsync(async { 42 }),
            ExprAwait(foo.await),
            ExprBinary(a + b),
            ExprBlock({ 42 }),
            ExprBreak(break 'foo),
            ExprCall(foo()),
            ExprCast(foo as usize),
            ExprClosure(|a, b| a + b),
            ExprConst(const { 42 }),
            ExprContinue(continue 'foo),
            ExprField(foo.bar),
            ExprForLoop(for a in b {}),
            ExprIf(if a { b } else { c }),
            ExprIndex(foo[bar]),
            ExprInfer(_),
            ExprLet(let a = b),
            ExprLit(42),
            ExprLoop(loop { break 'foo; }),
            ExprMacro(foo!()),
            ExprMatch(match foo { _ => 42 }),
            ExprMethodCall(foo.bar(baz)),
            ExprParen((42)),
            ExprPath(foo::bar),
            ExprRange(1..2),
            ExprReference(&mut foo),
            ExprRepeat([42; 4]),
            ExprReturn(return 42),
            ExprStruct(Foo { bar: baz }),
            ExprTry(foo?),
            ExprTryBlock(try { 42 }),
            ExprTuple((42, 42)),
            ExprUnary(!42),
            ExprUnsafe(unsafe { 42 }),
            ExprWhile(while true { 42 }),
            ExprYield(yield 42),
        )] })
        .expect("Failed to parse `allty`");

        assert!(allty.bool);
        assert_eq!(allty.String, "hello");
        assert_eq!(allty.char, 'c');
        assert_eq!(allty.f32, 4.2);
        assert_eq!(allty.f64, 6.9);
        assert_eq!(allty.u8, 1);
        assert_eq!(allty.i8, -1);
        assert_eq!(allty.u16, 2);
        assert_eq!(allty.i16, -2);
        assert_eq!(allty.u32, 3);
        assert_eq!(allty.i32, -3);
        assert_eq!(allty.u64, 4);
        assert_eq!(allty.i64, -4);
        assert_eq!(allty.usize, 5);
        assert_eq!(allty.isize, -5);
        assert_eq!(allty.Expr, parse_quote! { "lit expr" });
        assert_eq!(
            allty.AngleBracketedGenericArguments,
            parse_quote! { <String> }
        );
        assert_eq!(allty.ConstParam, parse_quote! { const FOO: usize });
        assert_eq!(allty.Abi, parse_quote! { extern "Foo" });
        assert_eq!(allty.BareFnArg, parse_quote! { &mut str });
        assert_eq!(allty.Ident, Ident::create("foo_ident"));
        assert_eq!(allty.Path, parse_quote! { ::core::Foo });
        assert_eq!(allty.Meta, parse_quote! { derive(Copy) });
        assert_eq!(allty.MetaList, parse_quote! { derive(Copy, Clone) });
        assert_eq!(allty.MetaNameValue, parse_quote! { foo = Bar });
        assert_eq!(allty.Visibility, parse_quote! { pub(crate) });
        assert_eq!(allty.Lifetime, parse_quote! { 'foo });
        assert_eq!(allty.LifetimeParam, parse_quote! { 'foo: 'bar + 'qux });
        assert_eq!(allty.BoundLifetimes, parse_quote! { for<'x> });
        assert_eq!(allty.TraitBound, parse_quote! { ?Sized });
        assert_eq!(allty.TypeParamBound, parse_quote! { ?Sized });
        assert_eq!(allty.TypeParam, parse_quote! { T: Clone });
        assert_eq!(allty.GenericParam, parse_quote! { T: Eq + PartialEq });
        assert_eq!(allty.WherePredicate, parse_quote! { T: Clone + Copy });
        assert_eq!(allty.Lit, parse_quote! { true });
        assert_eq!(allty.LitBool, parse_quote! { false });
        assert_eq!(allty.LitByteStr, parse_quote! { b"foo" });
        assert_eq!(allty.LitStr, parse_quote! { "foo" });
        assert_eq!(allty.LitByte, parse_quote! { b'@' });
        assert_eq!(allty.LitChar, parse_quote! { '_' });
        assert_eq!(allty.LitInt, parse_quote! { 42 });
        assert_eq!(allty.LitFloat, parse_quote! { 4.2 });
        assert_eq!(allty.Literal.to_string(), r#""Litter""#);
        assert_eq!(allty.Type, parse_quote! { bool });
        assert_eq!(allty.TypeArray, parse_quote! { [u8; 4] });
        assert_eq!(
            allty.TypeBareFn,
            parse_quote! { fn(&str, String) -> ::std::io::Result<()> }
        );
        assert_eq!(allty.TypeImplTrait, parse_quote! { impl Foo + Bar });
        assert_eq!(allty.TypeInfer, parse_quote! { _ });
        assert_eq!(allty.TypeMacro, parse_quote! { vec![] });
        assert_eq!(allty.TypeNever, parse_quote! { ! });
        assert_eq!(allty.TypeParen, parse_quote! { (u8) });
        assert_eq!(allty.TypePath, parse_quote! { foo::Bar });
        assert_eq!(allty.TypePtr, parse_quote! { *const u8 });
        assert_eq!(allty.TypeReference, parse_quote! { &u8 });
        assert_eq!(allty.TypeSlice, parse_quote! { [u8] });
        assert_eq!(allty.TypeTraitObject, parse_quote! { dyn Foo + Bar });
        assert_eq!(allty.TypeTuple, parse_quote! { (u8, u8) });
        assert_eq!(allty.ExprArray, parse_quote! { [1, 2, 3] });
        assert_eq!(allty.ExprAssign, parse_quote! { a = b });
        assert_eq!(allty.ExprAsync, parse_quote! { async { 42 } });
        assert_eq!(allty.ExprAwait, parse_quote! { foo.await });
        assert_eq!(allty.ExprBinary, parse_quote! { a + b });
        assert_eq!(allty.ExprBlock, parse_quote! { { 42 } });
        assert_eq!(allty.ExprBreak, parse_quote! { break 'foo });
        assert_eq!(allty.ExprCall, parse_quote! { foo() });
        assert_eq!(allty.ExprCast, parse_quote! { foo as usize });
        assert_eq!(allty.ExprClosure, parse_quote! { |a, b| a + b });
        assert_eq!(allty.ExprConst, parse_quote! { const { 42 } });
        assert_eq!(allty.ExprContinue, parse_quote! { continue 'foo });
        assert_eq!(allty.ExprField, parse_quote! { foo.bar });
        assert_eq!(allty.ExprForLoop, parse_quote! { for a in b {} });
        assert_eq!(allty.ExprIf, parse_quote! { if a { b } else { c } });
        assert_eq!(allty.ExprIndex, parse_quote! { foo[bar] });
        assert_eq!(allty.ExprInfer, parse_quote! { _ });
        assert_eq!(allty.ExprLet, parse_quote! { let a = b });
        assert_eq!(allty.ExprLit, parse_quote! { 42 });
        assert_eq!(allty.ExprLoop, parse_quote! { loop { break 'foo; } });
        assert_eq!(allty.ExprMacro, parse_quote! { foo!() });
        assert_eq!(allty.ExprMatch, parse_quote! { match foo { _ => 42 } });
        assert_eq!(allty.ExprMethodCall, parse_quote! { foo.bar(baz) });
        assert_eq!(allty.ExprParen, parse_quote! { (42) });
        assert_eq!(allty.ExprPath, parse_quote! { foo::bar });
        assert_eq!(allty.ExprRange, parse_quote! { 1..2 });
        assert_eq!(allty.ExprReference, parse_quote! { &mut foo });
        assert_eq!(allty.ExprRepeat, parse_quote! { [42; 4] });
        assert_eq!(allty.ExprReturn, parse_quote! { return 42 });
        assert_eq!(allty.ExprStruct, parse_quote! { Foo { bar: baz } });
        assert_eq!(allty.ExprTry, parse_quote! { foo? });
        assert_eq!(allty.ExprTryBlock, parse_quote! { try { 42 } });
        assert_eq!(allty.ExprTuple, parse_quote! { (42, 42) });
        assert_eq!(allty.ExprUnary, parse_quote! { !42 });
        assert_eq!(allty.ExprUnsafe, parse_quote! { unsafe { 42 } });
        assert_eq!(allty.ExprWhile, parse_quote! { while true { 42 } });
        assert_eq!(allty.ExprYield, parse_quote! { yield 42 });
    }

    #[test]
    fn punctuated() {
        #[derive(AttributeOptions, ParseOption)]
        struct Opts {
            data: Punctuated<u8, Token![,]>,
        }

        #[derive(AttributeOptions)]
        struct OptOpt {
            #[attr_opts(default(false))]
            inner: Opts,
        }

        let result =
            <Opts as AttributeOptions>::from_attr(parse_quote! { #[some_attr(data(1, 2, 3))] })
                .unwrap()
                .data
                .into_iter()
                .collect::<Vec<_>>();

        assert_eq!(&result, &[1, 2, 3]);

        let result = <OptOpt as AttributeOptions>::from_attr(
            parse_quote! { #[some_attr(inner(data(4, 5, 6)))] },
        )
        .unwrap()
        .inner
        .data
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(&result, &[4, 5, 6]);
    }
}
