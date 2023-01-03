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
