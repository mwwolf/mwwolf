use std::collections::HashMap;

use crate::*;

#[derive(new)]
pub struct SelectionService;

impl SelectionService {
    pub fn selection(&self, votes: Vec<Vote>) -> Vec<Id<Player>> {
        let counts = HashMap::<Id<Player>, usize>::new();
        let countes_map = votes.iter().fold(counts, |mut c, vote| {
            let cnt = c.get_mut(vote.target());
            if let Some(cnt) = cnt {
                *cnt += 1;
            } else {
                c.insert(vote.target().clone(), 1);
            }
            c
        });

        let mut max_count = 0;
        let targets = Vec::with_capacity(1);
        countes_map.iter().fold(targets, |mut t, vote| {
            if t.is_empty() || (t.get(0).is_some() && *vote.1 >= max_count) {
                t.push(vote.0.clone());
                max_count = *vote.1;
            }
            t
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        vec![
            Vote::new(
                Id::new("vote1".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player1".to_string()),
                Id::new("player2".to_string()),
            ),
        ]
        =>
        vec![
            Id::<Player>::new("player1".to_string()),
        ] ; "single_vote_single_result")]
    #[test_case(
        vec![
            Vote::new(
                Id::new("vote1".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player1".to_string()),
                Id::new("player2".to_string()),
            ),
            Vote::new(
                Id::new("vote2".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player1".to_string()),
                Id::new("player3".to_string()),
            ),
            Vote::new(
                Id::new("vote3".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player2".to_string()),
                Id::new("player4".to_string()),
            ),
        ]
        =>
        vec![
            Id::<Player>::new("player1".to_string()),
        ] ; "multi_vote_single_result")]
    #[test_case(
        vec![
            Vote::new(
                Id::new("vote1".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player1".to_string()),
                Id::new("player2".to_string()),
            ),
            Vote::new(
                Id::new("vote2".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player1".to_string()),
                Id::new("player3".to_string()),
            ),
            Vote::new(
                Id::new("vote3".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player2".to_string()),
                Id::new("player4".to_string()),
            ),
            Vote::new(
                Id::new("vote4".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player2".to_string()),
                Id::new("player5".to_string()),
            ),
            Vote::new(
                Id::new("vote5".to_string()),
                Id::new("talk1".to_string()),
                Id::new("player3".to_string()),
                Id::new("player6".to_string()),
            ),
        ]
        =>
        vec![
            Id::<Player>::new("player1".to_string()),
            Id::<Player>::new("player2".to_string()),
        ] ; "multi_vote_multi_result")]
    fn selection_works(votes: Vec<Vote>) -> Vec<Id<Player>> {
        let mut results = SelectionService::new().selection(votes);
        results.sort_by_key(|a| a.to_string());
        results
    }
}
