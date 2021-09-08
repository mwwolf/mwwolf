use super::*;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, NamedTupleFrom)]
pub struct ThemeKind(String);

impl ThemeKind {
    pub fn raw_kind(&self) -> &str {
        &self.0
    }
}

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

#[derive(new, Getters, Clone, PartialEq, Debug)]
pub struct Theme {
    id: Id<Theme>,
    kind: ThemeKind,
    first: Word,
    second: Word,
}

impl Theme {
    pub fn choice_word(&self, rng: &mut impl RngCore) -> (&Word, &Word) {
        if rng.gen() {
            (&self.first, &self.second)
        } else {
            (&self.second, &self.first)
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ThemeRepository {
    async fn find_by_kind(&self, kind: &ThemeKind) -> RepositoryResult<Vec<Theme>>;
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

    #[test_case(
        Theme::new(
            Id::new("theme1"),
            ThemeKind::try_new("test").unwrap(),
            Word::try_new("foo").unwrap(),
            Word::try_new("bar").unwrap(),
        ), rand::rngs::mock::StepRng::new(0, 1)
        =>
        (Word::try_new("bar").unwrap(), Word::try_new("foo").unwrap())
    )]
    #[test_case(
        Theme::new(
            Id::new("theme1"),
            ThemeKind::try_new("test").unwrap(),
            Word::try_new("foo2").unwrap(),
            Word::try_new("bar2").unwrap(),
        ), rand::rngs::mock::StepRng::new(1, 0)
        =>
        (Word::try_new("bar2").unwrap(), Word::try_new("foo2").unwrap())
    )]
    fn theme_choose_word_works(theme: Theme, mut rng: rand::rngs::mock::StepRng) -> (Word, Word) {
        let (word1, word2) = theme.choice_word(&mut rng);
        (word1.clone(), word2.clone())
    }
}
