use crate::Id;

#[derive(Debug, PartialEq)]
pub struct ThemeKind(String);

impl From<ThemeKind> for String {
    fn from(kind: ThemeKind) -> Self {
        kind.0
    }
}

#[derive(new, Getters)]
pub struct Theme {
    id: Id<Theme>,
    kind: ThemeKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(ThemeKind("food".into()) => "food")]
    fn thme_kind_into_test(kind: ThemeKind) -> String {
        kind.into()
    }
}
