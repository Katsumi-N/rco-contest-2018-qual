use proconio::{fastout, input};
use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::{time::{Instant}};
const COMMANDS: [&str; 4] = ["U", "D", "R", "L"];
const DX: [isize; 4] = [1, -1, 0, 0];
const DY: [isize; 4] = [0, 0, 1, -1];
const H: usize = 50;
const W: usize = 50;
const K: usize = 8;
const T: usize = 2500;

// 座標を保持する
#[derive(Debug,Clone,Copy)]
struct Coord {
    y: isize,
    x: isize,
}

impl Coord {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

#[derive(Debug,Clone)]
struct Board {
    id: usize,
    cell: Vec<Vec<char>>,
    character: Coord,
}

#[derive(Debug,Clone)]
struct State {
    boards: Vec<Board>,
    turn: usize,             // 現在のターン
    coin: usize, // ゲーム上で実際に得たスコア
    evaluated_score: usize,
    actions: Vec<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}

impl State {
    fn new(m: Vec<Vec<String>>) -> Self {
        let mut boards: Vec<Board> = vec![];
        for i in 0..K {
            let mut cell: Vec<Vec<char>> = m[i].iter().map(|x| x.chars().collect()).collect();
            let mut character = Coord::new(0, 0);
            for y in 0..H {
                for x in 0..W {
                    match cell[y][x] {
                        '@' => {
                            cell[y][x] = '.';
                            character.y = y as isize;
                            character.x = x as isize;
                        },
                        _ => (),
                    }
                }
            }
            let board: Board = Board { id: i, cell: cell, character: character };
            boards.push(board);
        }

        Self {
            boards,
            turn: 0,
            coin: 0,
            evaluated_score: 0,
            actions: vec![],
        }
    }

    // ゲームの終了判定
    fn is_done(&self) -> bool {
        self.turn == T
    }

    // 盤面評価はコインの数
    fn evaluate_score(&mut self) {
        self.evaluated_score = self.coin;
    }

    // 現在の状況でプレイヤーが可能な行動を全て取得する
    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];

        for action in 0..4 {
            let mut can_action = true;

            for board in &self.boards {
                let character = &board.character;
                let ty = character.y + DY[action];
                let tx = character.x + DX[action];
                if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize && board.cell[ty as usize][tx as usize] == 'x' {
                    can_action = false;
                    break;
                }
            }
            if can_action {
                actions.push(action);
            }
        }

        actions
    }

    // 指定したactionでゲームを1ターン進める
    fn advance(&mut self, action: usize) {
        for board in &mut self.boards { // all_boardsの名前に合わせました
            let character = &mut board.character;
            let ty = character.y + DY[action as usize];
            let tx = character.x + DX[action as usize];
            let cell = &mut board.cell[ty as usize][tx as usize];

            if *cell == '#' {
                continue;
            } else {
                if *cell == 'o' {
                    self.coin += 1;
                    *cell = '.';
                }
                character.y = ty;
                character.x = tx;
            }
        }
        self.turn += 1;
    }
    
}

fn chokudai_search_action(state: &State, beam_width: usize, beam_depth: usize) -> Vec<usize> {
    let mut beam = vec![BinaryHeap::new(); beam_depth + 1];
    beam[0].push(state.clone());

    let ti = Instant::now();
    while ti.elapsed().as_millis() < 3_000 {
        for t in 0..beam_depth {
            let mut now_beam = beam[t].clone();
            let next_beam = &mut beam[t + 1];

            for _ in 0..beam_width {
                if now_beam.is_empty() {
                    break;
                }

                let now_state = now_beam.peek().unwrap().clone();
                if now_state.is_done() {
                    break;
                }

                now_beam.pop();
                let legal_actions = now_state.legal_actions();
                for action in legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    next_state.actions.push(action);
                    next_beam.push(next_state);
                }
            }
        }
    }
    
    beam[beam_depth - 1].peek().unwrap().clone().actions
}

fn count_trap(vec_s: &Vec<String>) -> usize {
    let mut cnt: usize = 0;
    for s in vec_s {
        cnt += s.chars().filter(|&c| c == 'x').count();
    }
    cnt    
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: usize,
        w: usize,
        t: usize,
        maps: [[String; h]; n]
    }

    // 罠が少ない順にK個のマップを選ぶ
    let mut traps: Vec<_> = (0..n).map(|i| vec![i, count_trap(&maps[i])]).collect();
    traps.sort_by(|a, b| a[1].cmp(&b[1]));
    let m: Vec<_> = traps.iter().take(k).map(|x| x[0]).collect();
    println!("{}", m.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));

    let state = State::new(maps);
    let actions = chokudai_search_action(&state, 1, 2500);

    let ans = actions.iter().map(|a| COMMANDS[*a]).collect::<Vec<_>>().join("");
    println!("{}", ans);
}
