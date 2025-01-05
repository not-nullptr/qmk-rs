#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub score: i64,
    pub combo: usize, //连击数量
    pub high_combo: usize,
    pub eliminate_rows: usize,
}

impl alloc::fmt::Display for Record {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(
            f,
            "score: {}, highest combo: {}, rows eliminated: {}",
            self.score, self.high_combo, self.eliminate_rows
        )
    }
}

impl Record {
    pub fn new() -> Self {
        Self {
            score: 0,
            combo: 0,
            eliminate_rows: 0,
            high_combo: 0,
        }
    }

    pub fn compute(&mut self, rows_num: usize) {
        if rows_num == 0 {
            self.combo = 0;
            return;
        }
        for _ in 0..rows_num {
            self.combo_once()
        }
    }

    fn combo_once(&mut self) {
        self.score += 200 + (self.combo * 60) as i64;
        self.eliminate_rows += 1;
        self.combo += 1;
        // 计算历史最高连击
        if self.combo > self.high_combo {
            self.high_combo = self.combo
        }
    }
}
