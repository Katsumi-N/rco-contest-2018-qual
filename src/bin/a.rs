use proconio::{fastout, input};
use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const COMMANDS: [&str; 4] = ["U", "D", "R", "L"];
const DX: [isize; 4] = [1, -1, 0, 0];
const DY: [isize; 4] = [0, 0, 1, -1];
const H: usize = 50;
const W: usize = 50;

// 座標を保持する
#[derive(Clone,Copy)]
struct Coord {
    y: isize,
    x: isize,
}

impl Coord {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}
struct MazeState {
    points: Vec<Vec<isize>>,
    turn: usize,             // 現在のターン
    character: Coord,
    game_score: usize, // ゲーム上で実際に得たスコア
    evaluated_score: usize,
    first_action: isize,
    traps: Vec<Vec<usize>>,
}

impl Ord for MazeState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

impl PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MazeState {}

impl PartialEq for MazeState {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}

impl MazeState {
    fn new(m: Vec<String>) -> Self {
        let map_chars: Vec<Vec<char>> = m.into_iter().map(|x| x.chars().collect()).collect();
        let mut points: Vec<Vec<isize>> = vec![vec![0; W]; H];
        let mut character = Coord::new(0, 0);
        let mut traps: Vec<Vec<usize>> = vec![];
        for y in 0..H {
            for x in 0..W {
                match map_chars[y][x] {
                    '#' => {
                        points[y][x] = -1;
                    },
                    'o' => {
                        points[y][x] = 1;
                    },
                    'x' => {
                        traps.push(vec![y, x]);
                    },
                    '@' => {
                        character.x = x as isize;
                        character.y = y as isize;
                    },
                    _ => (),
                }
            }
        }
        Self {
            points,
            turn: 0,
            character,
            game_score: 0,
            evaluated_score: 0,
            first_action: -1,
            traps,
        }
    }

    // ゲームの終了判定
    fn is_done(&self) -> bool {
        for trap in &self.traps {
            if self.character.y == trap[0] as isize && self.character.x == trap[1] as isize {
                return true
            }
        }
        false
    }

    // 指定したactionでゲームを1ターン進める
    fn advance(&mut self, action: usize) {
        self.character.x += DX[action];
        self.character.y += DY[action];
        let point = &mut self.points[self.character.y as usize][self.character.x as usize];
        if *point > 0 {
            self.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
    }

    // 現在の状況でプレイヤーが可能な行動を全て取得する
    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 {
            let ty = self.character.y + DY[action];
            let tx = self.character.x + DX[action];
            if self.points[ty as usize][tx as usize] == -1 {
                continue;
            }
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                actions.push(action);
            }
        }
        actions
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }
}



fn random_command(t: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut c = String::from("");
    for _ in 0..t {
        c += COMMANDS[rng.gen_range(0, 4)];
    }
    c
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
    println!("{}", random_command(t));
}
