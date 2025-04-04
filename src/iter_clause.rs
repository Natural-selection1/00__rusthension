use syn::Expr;
use syn::Token;
use syn::parse::ParseStream;

/*-----------------IterClause------------------- */
#[derive(Debug)]
pub struct IterClause {
    pub for_in_clause: ForInClause,
    pub if_clause: Option<BareIfClause>,
}

impl syn::parse::Parse for IterClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut iter_clause = Self {
            for_in_clause: input.parse::<ForInClause>()?,
            if_clause: None,
        };

        if input.peek(syn::Token![if]) {
            iter_clause.if_clause = Some(input.parse::<BareIfClause>()?);
        }

        Ok(iter_clause)
    }
}

/*-----------------ForInClause------------------- */
#[derive(Debug)]
pub struct ForInClause {
    pub pat: syn::Pat,
    pub iterable: Expr,
}

impl syn::parse::Parse for ForInClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pat = input.call(syn::Pat::parse_single)?;
        input.parse::<Token![in]>()?;

        Ok(Self {
            pat,
            iterable: input.parse::<Expr>()?,
        })
    }
}

/*-----------------BareIfClause------------------- */
#[derive(Debug)]
pub struct BareIfClause {
    pub expr: Expr,
}

impl syn::parse::Parse for BareIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        Ok(Self {
            expr: input.parse::<Expr>()?,
        })
    }
}

/* ------------------------------------ */

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_iter_clause_parse() {
        // 测试基本的for-in子句解析
        let iter_clause: IterClause = parse_quote! {
            for x in vec![1, 2, 3]
        };
        assert!(matches!(iter_clause.for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(iter_clause.for_in_clause.iterable, Expr::Macro(_)));
        assert!(iter_clause.if_clause.is_none());
        eprintln!("IterClause基本for-in子句测试通过");

        // 测试带if条件的for-in子句解析
        let iter_clause: IterClause = parse_quote! {
            for x in items if x > 0
        };
        assert!(matches!(iter_clause.for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(iter_clause.for_in_clause.iterable, Expr::Path(_)));
        assert!(iter_clause.if_clause.is_some());
        if let Some(if_clause) = &iter_clause.if_clause {
            assert!(matches!(if_clause.expr, Expr::Binary(_)));
        }
        eprintln!("IterClause带if条件的for-in子句测试通过");

        // 测试复杂模式的for-in子句解析
        let iter_clause: IterClause = parse_quote! {
            for (x, y) in pairs
        };
        assert!(matches!(iter_clause.for_in_clause.pat, syn::Pat::Tuple(_)));
        assert!(matches!(iter_clause.for_in_clause.iterable, Expr::Path(_)));
        assert!(iter_clause.if_clause.is_none());
        eprintln!("IterClause复杂模式的for-in子句测试通过");

        // 测试复杂表达式的for-in子句解析
        let iter_clause: IterClause = parse_quote! {
            for x in vec.iter().map(|i| i * 2).filter(|i| i > 0) if x % 2 == 0
        };
        assert!(matches!(iter_clause.for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(
            iter_clause.for_in_clause.iterable,
            Expr::MethodCall(_)
        ));
        assert!(iter_clause.if_clause.is_some());
        if let Some(if_clause) = &iter_clause.if_clause {
            assert!(matches!(if_clause.expr, Expr::Binary(_)));
        }
        eprintln!("IterClause复杂表达式的for-in子句测试通过");
    }

    #[test]
    fn test_for_in_clause_parse() {
        // 测试基本的for-in子句解析
        let for_in_clause: ForInClause = parse_quote! {
            for x in items
        };
        assert!(matches!(for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(for_in_clause.iterable, Expr::Path(_)));
        eprintln!("ForInClause基本解析测试通过");

        // 测试复杂模式的for-in子句解析
        let for_in_clause: ForInClause = parse_quote! {
            for (a, b, c) in tuples
        };
        assert!(matches!(for_in_clause.pat, syn::Pat::Tuple(_)));
        assert!(matches!(for_in_clause.iterable, Expr::Path(_)));
        eprintln!("ForInClause复杂模式解析测试通过");

        // 测试复杂表达式的for-in子句解析
        let for_in_clause: ForInClause = parse_quote! {
            for x in (0..10).filter(|n| n % 2 == 0)
        };
        assert!(matches!(for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(for_in_clause.iterable, Expr::MethodCall(_)));
        eprintln!("ForInClause复杂表达式解析测试通过");
    }

    #[test]
    fn test_bare_if_clause_parse() {
        // 测试基本条件表达式解析
        let if_clause: BareIfClause = parse_quote! {
            if x > 0
        };
        assert!(matches!(if_clause.expr, Expr::Binary(_)));
        eprintln!("BareIfClause基本条件表达式测试通过");

        // 测试复杂条件表达式解析
        let if_clause: BareIfClause = parse_quote! {
            if x > 0 && y < 10 || z == 5
        };
        assert!(matches!(if_clause.expr, Expr::Binary(_)));
        eprintln!("BareIfClause复杂条件表达式测试通过");

        // 测试函数调用条件表达式解析
        let if_clause: BareIfClause = parse_quote! {
            if is_valid(x) && x.len() > 0
        };
        assert!(matches!(if_clause.expr, Expr::Binary(_)));
        eprintln!("BareIfClause函数调用条件表达式测试通过");
    }
}
