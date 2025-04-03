use syn::Token;
use syn::parse::ParseStream;

#[allow(unused)] //似乎是bug
use syn::parse_quote;

/*-----------------Mapping------------------- */
pub struct Mapping {
    pub left_expr: syn::Expr,
    pub right_expr: Option<MappingElse>,
}

impl syn::parse::Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut mapping = Mapping {
            left_expr: input.parse::<syn::Expr>()?,
            right_expr: None,
        };

        if input.peek(syn::Token![if]) {
            mapping.right_expr = Some(input.parse::<MappingElse>()?)
        }

        Ok(mapping)
    }
}

/*-----------------MappingElse------------------- */

pub struct MappingElse {
    pub conditions: syn::Expr,
    pub else_expr: syn::Expr,
}

impl syn::parse::Parse for MappingElse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let conditions = input.parse()?;
        input.parse::<Token![else]>()?;

        Ok(Self {
            conditions,
            else_expr: input.parse::<syn::Expr>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_parse() {
        // 测试基本表达式解析
        let mapping: Mapping = parse_quote! {
            x * 2 ** 2
        };
        assert!(matches!(mapping.left_expr, syn::Expr::Binary(_)));
        assert!(mapping.right_expr.is_none());
        eprintln!("Mapping基本表达式测试通过");
    }
    #[test]
    fn test_mapping_parse_with_condition() {
        // 测试带条件的Mapping解析
        let mapping: Mapping = parse_quote! {
            x * 2 if x > 0 && y < 10 else 0
        };
        assert!(matches!(mapping.left_expr, syn::Expr::Binary(_)));
        assert!(mapping.right_expr.is_some());
        if let Some(mapping_else) = &mapping.right_expr {
            assert!(matches!(mapping_else.conditions, syn::Expr::Binary(_)));
            assert!(matches!(mapping_else.else_expr, syn::Expr::Lit(_)));
        }
        eprintln!("Mapping带条件表达式测试通过");
    }
    #[test]
    fn test_mapping_parse_with_complex_expression() {
        // 测试复杂表达式解析
        let mapping: Mapping = parse_quote! {
            (x, y, z) if x > 0 && y < 10 else (0, 0, 0)
        };
        assert!(matches!(mapping.left_expr, syn::Expr::Tuple(_)));
        assert!(mapping.right_expr.is_some());
        eprintln!("Mapping复杂表达式测试通过");
    }
}
