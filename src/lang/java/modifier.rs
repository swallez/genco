//! Individual java modifier

use crate::java::Tokens;
use crate::{FormatTokens, Java};
use std::collections::BTreeSet;

/// A Java modifier.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Modifier {
    /// default
    Default,
    /// public
    Public,
    /// protected
    Protected,
    /// private
    Private,
    /// abstract
    Abstract,
    /// static
    Static,
    /// final
    Final,
    /// Native
    Native,
}

impl Modifier {
    /// Get the name of the modifier.
    pub fn name(&self) -> &'static str {
        use self::Modifier::*;

        match *self {
            Default => "default",
            Public => "public",
            Protected => "protected",
            Private => "private",
            Abstract => "abstract",
            Static => "static",
            Final => "final",
            Native => "native",
        }
    }
}

impl FormatTokens<Java> for Modifier {
    fn format_tokens(self, tokens: &mut Tokens) {
        tokens.append(self.name());
    }
}

impl FormatTokens<Java> for Vec<Modifier> {
    fn format_tokens(self, tokens: &mut Tokens) {
        let mut it = self.into_iter().collect::<BTreeSet<_>>().into_iter();

        if let Some(modifier) = it.next() {
            modifier.format_tokens(tokens);
        }

        for modifier in it {
            tokens.spacing();
            modifier.format_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Modifier;
    use crate as genco;
    use crate::{quote, Java, Tokens};

    #[test]
    fn test_vec() {
        use self::Modifier::*;
        let el: Tokens<Java> = quote!(#(vec![Public, Final, Static]));
        assert_eq!(
            Ok("public static final"),
            el.to_string().as_ref().map(|s| s.as_str())
        );
    }
}
