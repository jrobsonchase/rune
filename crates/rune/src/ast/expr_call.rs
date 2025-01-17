use crate::ast::prelude::*;

/// A function call `<expr>(<args>)`.
///
/// # Examples
///
/// ```
/// use rune::{ast, testing};
///
/// testing::roundtrip::<ast::ExprCall>("test()");
/// testing::roundtrip::<ast::ExprCall>("(foo::bar)()");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, ToTokens, Spanned, Opaque)]
#[non_exhaustive]
pub struct ExprCall {
    /// Opaque identifier related with call.
    #[rune(id)]
    pub(crate) id: Id,
    /// Attributes associated with expression.
    #[rune(iter)]
    pub attributes: Vec<ast::Attribute>,
    /// The name of the function being called.
    pub expr: Box<ast::Expr>,
    /// The arguments of the function call.
    pub args: ast::Parenthesized<ast::Expr, T![,]>,
}

impl ExprCall {
    /// Get the target of the call expression.
    pub(crate) fn target(&self) -> &ast::Expr {
        if let ast::Expr::FieldAccess(access) = &*self.expr {
            return &access.expr;
        }

        &self.expr
    }
}

expr_parse!(Call, ExprCall, "call expression");
