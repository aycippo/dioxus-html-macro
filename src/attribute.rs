use crate::prelude::*;
/// An attribute can have one of the
/// following two structures:
/// * `ident=value`
/// * `"custom-attr"="string-lit"
pub struct Attribute {
    pub name: AttributeIdent,
    pub equals: Token![=],
    pub value: RsxExpr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let equals = input.parse()?;
        let value = input.parse()?;
        let attr = Attribute {
            name,
            equals,
            value,
        };

        attr.validate()?;

        Ok(attr)
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attribute { name, value, .. } = self;

        tokens.extend(quote! {
            #name: #value
        });
    }
}

impl Attribute {
    fn validate(&self) -> Result<()> {
        if !self.name.is_ident() && !self.value.is_str() {
            return Err(Error::new(self.value.span(), "expected a string literal."));
        }

        Ok(())
    }
}

/// represents either an identifier or a string literal
pub enum AttributeIdent {
    Ident(Ident),
    LitStr(LitStr),
}

impl Parse for AttributeIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok({
            if input.peek(LitStr) {
                Self::LitStr(input.parse()?)
            } else {
                Self::Ident(input.parse()?)
            }
        })
    }
}

impl ToTokens for AttributeIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend({
            match self {
                Self::Ident(ident) => quote!(#ident),
                Self::LitStr(litstr) => quote!(#litstr),
            }
        });
    }
}

impl AttributeIdent {
    pub fn is_ident(&self) -> bool {
        matches!(&self, Self::Ident(_))
    }
}

pub struct Attributes(Vec<Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = vec![];

        loop {
            if input.peek(Token![/]) || input.peek(Token![>]) {
                break Ok(Attributes(attrs));
            }
            let attr = input.parse()?;
            attrs.push(attr);
        }
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attributes(attributes) = self;

        tokens.extend(quote! {#(#attributes,)*})
    }
}
