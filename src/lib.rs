use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Op {
    Add,
    Subtract,
    Multipy,
    Divide,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Op::Add => "+",
            &Op::Subtract => "-",
            &Op::Multipy => "*",
            &Op::Divide => "/",
        };
        write!(f, "{}", &s)
    }
}

pub struct Value {
    pub number: u8,
    pub power: u32,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.power == 1 {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{}^{}", self.number, self.power)
        }
    }
}

pub struct Answer {
    pub x: Value,
    pub op1: Op,
    pub y: Value,
    pub op2: Op,
    pub z: Value,
}

impl Answer {
    pub fn from(
        x: &u8,
        x_power: &u32,
        op1: &Op,
        y: &u8,
        y_power: &u32,
        op2: &Op,
        z: &u8,
        z_power: &u32,
    ) -> Answer {
        Answer {
            x: Value {
                number: x.clone(),
                power: x_power.clone(),
            },
            op1: op1.clone(),
            y: Value {
                number: y.clone(),
                power: y_power.clone(),
            },
            op2: op2.clone(),
            z: Value {
                number: z.clone(),
                power: z_power.clone(),
            },
        }
    }
}

pub type Board = HashMap<u8, Answer>;

const BOARD_SIZE: u8 = 36;

pub fn display(results: &Board) -> String {
    let mut numbers: Vec<&u8> = results.keys().collect();
    numbers.sort();
    let mut result = String::new();
    for number in numbers.iter() {
        let answer = results.get(number).unwrap();
        result.push_str(&format!(
            "{:3} {} {:3} {} {:3} = {}\n",
            answer.x.to_string(),
            answer.op1,
            answer.y.to_string(),
            answer.op2,
            answer.z.to_string(),
            number
        ));
    }
    result
}

pub fn fill_board(n1: u8, n2: u8, n3: u8) -> Board {
    let mut results: Board = HashMap::with_capacity(BOARD_SIZE as usize);
    let numbers = vec![n1, n2, n3];
    let perms = permutations(&numbers);
    let powers = [1, 0, 2, 3];
    let ops = [Op::Add, Op::Subtract, Op::Multipy, Op::Divide];
    for op1 in ops.iter() {
        for op2 in ops.iter() {
            for &(x, y, z) in &perms {
                for x_power in &powers {
                    for y_power in &powers {
                        for z_power in &powers {
                            let x_final = x.pow(*x_power);
                            let y_final = y.pow(*y_power);
                            let z_final = z.pow(*z_power);
                            let answer = op(x_final, op1, y_final);
                            let answer = op(answer, op2, z_final);
                            if answer != 0 && answer <= BOARD_SIZE && !results.contains_key(&answer)
                            {
                                results.insert(
                                    answer,
                                    Answer::from(&x, x_power, op1, &y, y_power, op2, &z, z_power),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    results
}

pub fn get_missing(results: &Board) -> Vec<u8> {
    let found: HashSet<u8> = HashSet::from_iter(results.keys().cloned());
    let possible: HashSet<u8> = HashSet::from_iter(1..BOARD_SIZE + 1);
    let mut missing: Vec<u8> = possible.difference(&found).cloned().collect();
    missing.sort();
    missing
}

fn op(n1: u8, op: &Op, n2: u8) -> u8 {
    match op {
        &Op::Add => n1.saturating_add(n2),
        &Op::Subtract => n1.saturating_sub(n2),
        &Op::Multipy => n1.saturating_mul(n2),
        &Op::Divide => {
            if n2 == 0 {
                return 0;
            }
            if n1.checked_rem(n2).expect("div by 0 not possible") != 0 {
                return 0;
            }
            n1.checked_div(n2).expect("div by 0 not possible")
        }
    }
}

fn permutations(numbers: &Vec<u8>) -> Vec<(u8, u8, u8)> {
    let mut perms: Vec<(u8, u8, u8)> = vec![];
    for (x_index, x) in numbers.iter().enumerate() {
        let mut numbers_without_x = numbers.clone();
        numbers_without_x.remove(x_index);
        for (y_index, y) in numbers_without_x.iter().enumerate() {
            let mut numbers_without_x_and_y = numbers_without_x.clone();
            numbers_without_x_and_y.remove(y_index);
            let z = &numbers_without_x_and_y[0];
            perms.push((*x, *y, *z));
        }
    }
    perms
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1_1() {
        let results = fill_board(1, 1, 1);
        let missing = get_missing(&results);
        assert_eq!(
            vec![
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
            ],
            missing
        );
    }

    #[test]
    fn test_2_2_2() {
        let results = fill_board(2, 2, 2);
        let missing = get_missing(&results);
        assert_eq!(vec![19, 21, 22, 23, 25, 26, 27, 29, 35], missing);
    }

    #[test]
    fn test_3_5_1() {
        let results = fill_board(3, 5, 1);
        let missing = get_missing(&results);
        assert_eq!(vec![19, 30], missing);
    }

    #[test]
    fn test_3_4_2() {
        let results = fill_board(3, 4, 2);
        let answer = results.get(&22).unwrap();
        assert_eq!(4, answer.x.number);
        assert_eq!(3, answer.x.power);
        assert_eq!(Op::Add, answer.op1);
        assert_eq!(2, answer.y.number);
        assert_eq!(1, answer.y.power);
        assert_eq!(Op::Divide, answer.op2);
        assert_eq!(3, answer.z.number);
        assert_eq!(1, answer.z.power);
    }
}
