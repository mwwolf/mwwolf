use std::collections::HashMap;

use crate::*;

#[derive(new)]
pub struct SelectionService;

impl SelectionService {
    pub fn selection(&self, votes: Vec<Vote>) -> Vec<Id<Player>> {
        let mut max_count = 1;
        let mut counts = HashMap::<Id<Player>, usize>::new();
        for vote in votes.iter() {
            if let Some(cnt) = counts.get_mut(vote.target()) {
                *cnt += 1;
                if *cnt > max_count {
                    max_count = *cnt;
                }
            } else {
                counts.insert(vote.target().clone(), 1);
            }
        }

        counts
            .into_iter()
            .filter(|(_, c)| c == &max_count)
            .map(|(p, _)| p)
            .collect()
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
