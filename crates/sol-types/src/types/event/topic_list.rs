use crate::{token::WordToken, Error, Result, SolType};
use alloc::borrow::Cow;
use sealed::Sealed;

/// A `TopicList` represents the topics of a Solidity event.
///
/// This trait is implemented only on tuples of arity up to 4. The tuples must
/// contain only [`SolType`]s where the token is a [`WordToken`], and as such
/// it is sealed to prevent prevent incorrect downstream implementations.
///
/// See the [Solidity event ABI specification][solevent] for more details on how
/// events' topics are encoded.
///
/// [solevent]: https://docs.soliditylang.org/en/latest/abi-spec.html#events
pub trait TopicList: SolType + Sealed {
    /// The number of topics.
    const COUNT: usize;

    /// Detokenize the topics into a tuple of rust types.
    ///
    /// This function accepts an iterator of `WordToken`.
    fn detokenize<I, D>(topics: I) -> Result<Self::RustType>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! impl_topic_list_tuples {
    ($($c:literal => $($t:ident),*;)+) => {$(
        impl<'aa, $($t: SolType<TokenType<'aa> = WordToken>,)*> sealed::Sealed for ($($t,)*) {}
        impl<'aa, $($t: SolType<TokenType<'aa> = WordToken>,)*> TopicList for ($($t,)*) {
            const COUNT: usize = $c;

            fn detokenize<I, D>(topics: I) -> Result<Self::RustType>
            where
                I: IntoIterator<Item = D>,
                D: Into<WordToken>
            {
                let err = || Error::Other(Cow::Borrowed("topic list length mismatch"));
                let mut iter = topics.into_iter().map(Into::into);
                Ok(($(
                    iter.next().ok_or_else(err).map(|tok| <$t>::detokenize(tok))?,
                )*))
            }
        }
    )+};
}

impl sealed::Sealed for () {}
impl TopicList for () {
    const COUNT: usize = 0;

    #[inline]
    fn detokenize<I, D>(_: I) -> Result<Self::RustType>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        Ok(())
    }
}

impl_topic_list_tuples! {
    1 => T;
    2 => T, U;
    3 => T, U, V;
    4 => T, U, V, W;
}
