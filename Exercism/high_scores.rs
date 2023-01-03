// Instructions
// Manage a game player's High Score list.

// Your task is to build a high-score component of the classic Frogger game, one of the highest selling and addictive games of all time, 
// and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

// Consider retaining a reference to scores in the struct - copying is not necessary. You will require some lifetime annotations, though.


#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result;
        
        if self.scores.len() <= 3 {
            result = self.scores.to_vec();
            result.sort_unstable_by(|a, b| b.cmp(a));
        } else {
            result = vec![0; 3];
            let mut mut_scores = self.scores.to_vec();
            mut_scores.sort();
            for i in 0..3{
            result[i] = mut_scores.pop().unwrap();
            }
        }
        result
    }
}
