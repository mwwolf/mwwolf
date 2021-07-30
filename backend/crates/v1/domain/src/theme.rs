use crate::*;

#[derive(Debug, PartialEq, NamedTupleFrom)]
pub struct ThemeKind(String);

impl ThemeKind {
    pub fn try_new(kind: impl Into<String>) -> DomainResult<Self> {
        let kind = kind.into();
        if kind.is_empty() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "kind should not be blank",
            ))
        } else {
            Ok(Self(kind))
        }
    }
}

#[derive(Debug, PartialEq, NamedTupleFrom, Clone)]
pub struct Word(String);
impl Word {
    pub fn try_new(word: impl Into<String>) -> DomainResult<Self> {
        let word = word.into();
        if word.is_empty() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "word should not be blank",
            ))
        } else {
            Ok(Self(word))
        }
    }
}

#[derive(new, Getters)]
pub struct Theme {
    id: Id<Theme>,
    kind: ThemeKind,
    first: Word,
    second: Word,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge"=> Ok(ThemeKind("hoge".into())))]
    #[test_case(""=> Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "kind should not be blank",
            )))]
    fn theme_kind_try_new_works(kind: impl Into<String>) -> DomainResult<ThemeKind> {
        ThemeKind::try_new(kind)
    }

    #[test_case("hoge"=> Ok(Word("hoge".into())))]
    #[test_case(""=> Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "word should not be blank",
            )))]
    fn word_try_new_works(word: impl Into<String>) -> DomainResult<Word> {
        Word::try_new(word)
    }
}
