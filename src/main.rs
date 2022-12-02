use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

const WINPTS: i32 = 6;
const DRAWPTS: i32 = 3;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const DRAW: i32 = 2;
const WIN: i32 = 3;


fn main() -> std::io::Result<()> {
    let score_map : HashMap<String, i32> = HashMap::from([
        ("A".to_string(),1),
        ("B".to_string(),2),
        ("C".to_string(),3),
        ("X".to_string(),1),
        ("Y".to_string(),2),
        ("Z".to_string(),3),
    ]);
    test_calc();
    let file = File::open("./src/inputo").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut p1points: i32 = 0;
    let mut p2points: i32 = 0;
    for line in reader.lines() {
        if let Ok(lin) = line{
            let elfmove = lin.chars().nth(0).expect("Cannot convert!").to_string();
            let youmove = lin.chars().nth(2).expect("Cannot convert!").to_string();
            let elfval: i32 = *score_map.get(&elfmove.clone()).expect("VALUE NOT FOUND!!");
            let youval: i32 = *score_map.get(&youmove.clone()).expect("VALUE NOT FOUND!!");
            let rpoints: i32 = point_calc_p1(elfval,youval);
            p1points = p1points + rpoints;
            let r2points: i32 = point_calc_p2(elfval,youval);
            p2points = p2points + r2points;
        }   
    }
    println!("Total P1 points: {:?}", p1points);
    println!("Total P2 points: {:?}", p2points);
    Ok(())
}

fn point_calc_p2(elf: i32, res: i32) -> i32 {
    if res == DRAW {
        return elf + DRAWPTS
    } else if res == WIN {
        if elf == ROCK {
            return PAPER + WINPTS
        } else if elf == PAPER {
            return SCISSORS + WINPTS
        } else {
            return ROCK + WINPTS
        }
    } else {
        if elf == ROCK {
            return SCISSORS
        } else if elf == PAPER {
            return ROCK
        } else {
            return PAPER
        }
    }
}

fn point_calc_p1(elf: i32, you: i32) -> i32 {
    if you == elf {
        return you + DRAWPTS
    }
    if you == ROCK {
        if elf == PAPER {
            return you
        } else {
            return you + WINPTS
        }
    } else if you == PAPER {
        if elf == SCISSORS {
            return you
        } else {
            return you + WINPTS
        }
    } else {
        if elf == ROCK {
            return you
        } else {
            return you + WINPTS
        }
    }
}

fn test_calc() {
    assert_eq!(point_calc_p1(1,1),4);//TIE
    assert_eq!(point_calc_p1(1,2),8);//WIN
    assert_eq!(point_calc_p1(1,3),3);//LOSE
    assert_eq!(point_calc_p1(2,1),1);//LOSE
    assert_eq!(point_calc_p1(2,2),5);//TIE
    assert_eq!(point_calc_p1(2,3),9);//WIN
    assert_eq!(point_calc_p1(3,1),7);//WIN
    assert_eq!(point_calc_p1(3,2),2);//LOSE
    assert_eq!(point_calc_p1(3,3),6);//TIE
}