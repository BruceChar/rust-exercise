#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl <'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|a,b| b.cmp(a));
        v.truncate(3);
        v
    }
}
