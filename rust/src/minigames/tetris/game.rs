use super::{bricks::*, record::Record};
use alloc::string::ToString;
use alloc::vec;
use alloc::{collections::vec_deque::VecDeque, string::String, vec::Vec};

use super::config::{TETRIS_HEIGHT, TETRIS_WIDTH};

#[derive(Debug, PartialEq, Clone)]
pub enum GameStatus {
    Running,
    Pause,
    Accelerative,
    Exit(String),
}

#[derive(PartialEq, Eq)]
pub enum InGameStatus {
    GameJustOver,
    KeepDropping,
    FinishDropping,
}

pub enum ControlLimit {
    CantLeft,
    CantRight,
    CantLeftAndRight,
}

#[derive(Debug, Clone)]
pub struct Tetris {
    pub board: Board,
    pub status: GameStatus,
    pub now_brick: Option<Brick>,
    pub now_brick_position: (usize, usize),
    pub following_bricks: VecDeque<Brick>,
    pub record: Record,
}

impl Tetris {
    pub fn new() -> Self {
        let w = TETRIS_WIDTH;
        let h = TETRIS_HEIGHT;
        let mut q = VecDeque::new();
        for _ in 0..3 {
            q.push_back(Brick::random());
        }
        let board = Board::new(w, h);
        let c = board.center;
        Self {
            board,
            status: GameStatus::Pause,
            now_brick_position: (c, 0),
            following_bricks: q,
            now_brick: None,
            record: Record::new(),
        }
    }

    pub fn get_shadow(&self) -> Vec<(isize, isize)> {
        let mut now_poss = self.get_absolute();
        while self.try_collapse(now_poss.clone()).is_none() {
            for i in 0..now_poss.len() {
                now_poss[i].1 += 1;
            }
        }
        now_poss
    }

    fn line_full(line: &Line) -> bool {
        line.iter().all(|x| x.0)
    }

    // instance method
    fn add_next_brick(&mut self) {
        self.following_bricks.push_back(Brick::random())
    }

    fn combout(&mut self) -> usize {
        // compute how many lines were combed.
        let mut combo_count = 0;
        for i in 0..self.board.height {
            if Self::line_full(&self.board.datas[i]) {
                combo_count += 1;
                //符合消除条件。
                self.board.datas.remove(i); //这一层消除
                self.board
                    .datas
                    .insert(0, vec![Unit(false); self.board.width]); //添加新层到最前面。
            }
        }
        combo_count
    }
    pub fn try_rotate(&mut self) -> bool {
        if let Some(brick) = &mut self.now_brick {
            let old = brick.clone();
            brick.rotate();
            if !self.is_legal_positions() || self.is_overlapped() {
                self.now_brick = Some(old); //还原
                return false;
            }
            return true;
        }
        return false;
    }
    // is_legal_positions 是否越界
    pub fn is_legal_positions(&self) -> bool {
        let mut is_legal = true;
        for (x, y) in self.get_absolute() {
            if x < 0 || (x > self.board.width as isize - 1) {
                // x横向越界!
                is_legal = false
            }
            if y > self.board.height as isize - 1 {
                // y 出现不可能的更高值
                is_legal = false
            }
        }
        is_legal
    }
    pub fn is_overlapped(&self) -> bool {
        // 只有旋转需要重叠检验。左右移动使用limits检验，下落使用collapse检验.
        for (x, y) in self.get_absolute() {
            // 不考虑负y
            if y >= 0 {
                if self.board.datas[y as usize][x as usize].0 {
                    return true;
                }
            }
        }
        false
    }

    // -----------------EVENT--------------------
    pub fn event_rotate(&mut self) {
        self.try_rotate();
    }

    pub fn event_left(&mut self) {
        match self.limited() {
            Some(limit) => match limit {
                ControlLimit::CantLeft => return,
                ControlLimit::CantLeftAndRight => return,
                ControlLimit::CantRight => {}
            },
            None => {}
        }

        self.now_brick_position.0 -= 1;
    }

    pub fn event_right(&mut self) {
        match self.limited() {
            Some(limit) => match limit {
                ControlLimit::CantRight => return,
                ControlLimit::CantLeftAndRight => return,
                ControlLimit::CantLeft => {}
            },
            None => {}
        }
        self.now_brick_position.0 += 1;
    }
    pub fn event_quit(&mut self) {
        self.status = GameStatus::Exit("keyboard quit".to_string());
    }

    pub fn event_sink(&mut self) {
        // 持续掉掉落
        // 这里不需要担心内部的游戏结束触发。机制。如果结束，则游戏Status成为Exit，游戏循环内通过判断则结束游戏。
        let mut coounter = 0;
        while self.down_settle() == InGameStatus::KeepDropping {
            coounter += 1;
        }
        self.record.score += coounter;
    }

    pub fn event_accelerate(&mut self) {
        self.down_settle();
        self.record.score += 1;
    }
    // ---------------EVENT END--------------------

    pub fn limited(&self) -> Option<ControlLimit> {
        //是否贴着左右的Unit 用于限制左右移动碰撞箱
        let absolute_positions = self.get_absolute();
        // 尝试探测
        let mut cant_l = false;
        let mut cant_r = false;
        for e in &absolute_positions {
            let &(x, y) = e;
            //防止越界
            if y >= 0 {
                // 左边
                if x == 0 || self.board.datas[y as usize][x as usize - 1].0 {
                    cant_l = true
                }
                // 右边
                if x == (self.board.width - 1) as isize
                    || self.board.datas[y as usize][x as usize + 1].0
                {
                    cant_r = true
                }
            }
        }
        match (cant_l, cant_r) {
            (true, true) => Some(ControlLimit::CantLeftAndRight),
            (true, false) => Some(ControlLimit::CantLeft),
            (false, true) => Some(ControlLimit::CantRight),
            (false, false) => None,
        }
    }

    pub fn get_absolute(&self) -> Vec<(isize, isize)> {
        match self.now_brick.as_ref() {
            Some(e) => e.pixels_info(
                self.now_brick_position.0 as isize,
                self.now_brick_position.1 as isize,
            ),
            None => vec![],
        }
    }

    fn collapse(&mut self, poss: Vec<(isize, isize)>) {
        for e in &poss {
            let y = e.1;
            if y >= 0 {
                self.board.datas[e.1 as usize][e.0 as usize] = Unit(true)
            }
        }
    }
    fn try_collapse(&self, poss: Vec<(isize, isize)>) -> Option<Vec<(isize, isize)>> {
        // 绝对位置poss
        // 尝试碰撞
        let mut can_collapse = false;
        for e in &poss {
            let &(x, y) = e;
            if y >= 0 {
                if y == self.board.height as isize - 1 {
                    // 碰到地板
                    can_collapse = true;
                    break;
                }

                if self.board.datas[y as usize + 1][x as usize].0 {
                    // 碰到了实体方块
                    can_collapse = true;
                    break;
                }
            }
        }
        if can_collapse {
            return Some(poss);
        } else {
            return None;
        }
    }

    fn try_down(&mut self) -> InGameStatus {
        if let Some(poss) = self.try_collapse(self.get_absolute()) {
            self.collapse(poss.clone());
            let new_poss: Vec<(isize, isize)> = self.get_absolute();
            // 判断游戏是否结束
            for (_, y) in new_poss {
                if y < 0 {
                    // 确定结束了
                    self.status = GameStatus::Exit("full".to_string());
                    return InGameStatus::GameJustOver;
                }
            }
            // 这里是完成💥（碰撞）同时还没有游戏结束。
            return InGameStatus::FinishDropping;
        }
        return InGameStatus::KeepDropping;
    }

    // 结算
    //  /游戏结束 /完成掉落刚刚落地 /继续掉落中
    fn down_settle(&mut self) -> InGameStatus {
        let down_result = self.try_down();
        match down_result {
            InGameStatus::FinishDropping => {
                let times = self.combout(); //计算消除的行数
                self.record.compute(times); //记录对应的分数
                self.new_small_run(); //召唤新的砖块.
            }
            InGameStatus::KeepDropping => {
                self.now_brick_position.1 += 1;
            }
            InGameStatus::GameJustOver => {}
        }
        down_result
    }

    fn new_small_run(&mut self) {
        let new_brick = self.following_bricks.pop_front().unwrap();
        self.now_brick = Some(new_brick);
        self.add_next_brick();
        //开始第二个
        self.now_brick_position = (self.board.center, 0);
        // 计算是否重叠，否则直接结束游戏.
        if self.is_overlapped() {
            self.status = GameStatus::Exit("overlap".to_string());
        }
    }

    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.new_small_run();
    }
    pub fn update(&mut self) {
        self.down_settle();
    }
}

#[derive(Debug, Clone)]
pub struct Unit(pub bool);
pub type Line = Vec<Unit>;
#[derive(Debug, Clone)]
pub struct Board {
    pub center: usize,
    pub width: usize,
    pub height: usize,
    pub datas: Vec<Line>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut datas = vec![];
        for _ in 0..height {
            datas.push(vec![Unit(false); width])
        }
        Self {
            width,
            height,
            datas,
            center: width / 2,
        }
    }
}
