//! A single element

use crate::{Formatter, ItemStr, Lang, LangBox, LangItem as _};
use std::fmt;
use std::rc::Rc;

/// A single element in a set of tokens.
pub enum Item<L>
where
    L: Lang,
{
    /// A literal item.
    /// Is added as a raw string to the stream of tokens.
    Literal(ItemStr),
    /// A quoted string.
    ///
    /// The string content is quoted with the language-specific [quoting method].
    /// [quoting method]: Lang::quote_string
    Quoted(ItemStr),
    /// A language-specific boxed item.
    LangBox(LangBox<L>),
    /// A language-specific boxed item that is not rendered.
    Registered(LangBox<L>),
    /// Push a new line unless the current line is empty.
    Push,
    /// Unconditionally push a line.
    Line,
    /// Spacing between language items. Typically a single space.
    ///
    /// Multiple spacings in sequence are collapsed into one.
    /// A spacing does nothing if at the beginning of a line.
    Spacing,
    /// Indent one step.
    Indent,
    /// Unindent one step.
    Unindent,
}

impl<L> Item<L>
where
    L: Lang,
{
    /// Format the given element.
    pub fn format(&self, out: &mut Formatter, config: &mut L::Config, level: usize) -> fmt::Result {
        use self::Item::*;

        match *self {
            Registered(_) => {}
            Literal(ref literal) => {
                out.write_str(literal.as_ref())?;
            }
            Quoted(ref literal) => {
                L::quote_string(out, literal.as_ref())?;
            }
            LangBox(ref lang) => {
                lang.format(out, config, level)?;
            }
            // whitespace below
            Push => {
                out.new_line_unless_empty()?;
            }
            Line => {
                out.new_line()?;
            }
            Spacing => {
                out.write_str(" ")?;
            }
            Indent => {
                out.indent();
                out.new_line_unless_empty()?;
            }
            Unindent => {
                out.unindent();
                out.new_line_unless_empty()?;
            }
        }

        Ok(())
    }
}

impl<L> fmt::Debug for Item<L>
where
    L: Lang,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(s) => write!(fmt, "Literal({:?})", s),
            Self::Quoted(s) => write!(fmt, "Quoted({:?})", s),
            Self::LangBox(item) => write!(fmt, "LangBox({:?})", item),
            Self::Registered(item) => write!(fmt, "Registered({:?})", item),
            Self::Push => write!(fmt, "Push"),
            Self::Line => write!(fmt, "Line"),
            Self::Spacing => write!(fmt, "Spacing"),
            Self::Indent => write!(fmt, "Indent"),
            Self::Unindent => write!(fmt, "Unindent"),
        }
    }
}

impl<L> From<String> for Item<L>
where
    L: Lang,
{
    fn from(value: String) -> Self {
        Item::Literal(value.into())
    }
}

impl<'a, L> From<&'a str> for Item<L>
where
    L: Lang,
{
    fn from(value: &'a str) -> Self {
        Item::Literal(value.into())
    }
}

impl<L> From<Rc<String>> for Item<L>
where
    L: Lang,
{
    fn from(value: Rc<String>) -> Self {
        Item::Literal(value.into())
    }
}

impl<L> From<ItemStr> for Item<L>
where
    L: Lang,
{
    fn from(value: ItemStr) -> Self {
        Item::Literal(value)
    }
}

impl<'a, L> From<&'a Item<L>> for Item<L>
where
    L: Lang,
{
    fn from(value: &'a Item<L>) -> Self {
        value.clone()
    }
}

impl<L> From<Rc<Item<L>>> for Item<L>
where
    L: Lang,
{
    fn from(value: Rc<Item<L>>) -> Self {
        (*value).clone()
    }
}

impl<L> Clone for Item<L>
where
    L: Lang,
{
    fn clone(&self) -> Self {
        match self {
            Self::Literal(literal) => Self::Literal(literal.clone()),
            Self::Quoted(quoted) => Self::Quoted(quoted.clone()),
            Self::LangBox(lang) => Self::LangBox(lang.clone()),
            Self::Registered(lang) => Self::Registered(lang.clone()),
            Self::Push => Self::Push,
            Self::Line => Self::Line,
            Self::Spacing => Self::Spacing,
            Self::Indent => Self::Indent,
            Self::Unindent => Self::Unindent,
        }
    }
}