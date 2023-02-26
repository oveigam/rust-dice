use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::thread;
use std::{collections::HashMap, time::Instant};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

static GAMES: u64 = 1000000000 as u64;
static THREADS: u64 = 32;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, Display)]
enum Hand {
    Violín,
    Pareja,
    DoblePareja,
    Trío,
    EscaleraMenor,
    EscaleraMayor,
    Full,
    Poker,
    Repoker,
}

struct Percentage {
    hand: Hand,
    normal: f64,
    ace: f64,
    diff: f64,
}

fn main() {
    let start = Instant::now();
    let normal_play = simulation(false);
    let ace_play = simulation(true);

    let mut percentages: Vec<Percentage> = Vec::new();

    for hand in Hand::iter() {
        let mut count_normal: u32 = 0;
        let mut count_ace: u32 = 0;
        for result in normal_play.iter() {
            let c = result.get(&hand).unwrap();
            count_normal = count_normal + c;
        }
        for result in ace_play.iter() {
            let c = result.get(&hand).unwrap();
            count_ace = count_ace + c;
        }

        let p_normal = count_normal as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_ace = count_ace as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_diff = p_ace - p_normal;

        percentages.push(Percentage {
            hand,
            normal: p_normal,
            ace: p_ace,
            diff: p_diff,
        });
    }

    println!("\n\n\n\n");
    println!(
        "Partidas simuladas:  {}",
        (GAMES * THREADS).to_formatted_string(&Locale::fr)
    );
    println!("Tiempo de ejecución: {:?}", start.elapsed());
    println!("");

    print!("       ");
    for p in &percentages {
        print!(" | {0: <15}", p.hand)
    }
    print!("\nnormal ");
    for p in &percentages {
        print!(" | {0: <15}", format!("{:.4}%", p.normal))
    }
    print!("\nguardar");
    for p in &percentages {
        print!(" | {0: <15}", format!("{:.4}%", p.ace))
    }
    print!("\ndiff   ");
    for p in &percentages {
        if p.diff > 0.0 {
            print!(" | +{0: <14}", format!("{:.4}", p.diff))
        } else {
            print!(" | {0: <15}", format!("{:.4}", p.diff))
        }
    }

    println!("\n\n\n\n");
}

fn simulation(keep_ace: bool) -> Vec<HashMap<Hand, u32>> {
    let mut handles = Vec::new();
    let mut results = Vec::new();

    for _i in 0..THREADS {
        let h = thread::spawn(move || play(keep_ace));
        handles.push(h);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}

fn get_hand(dice: [i8; 5]) -> Hand {
    let mut face_count: [i8; 6] = [0, 0, 0, 0, 0, 0]; // index represents the face

    for face in dice {
        face_count[face as usize] = face_count[face as usize] + 1;
    }

    let mut trio: i8 = -1;
    let mut pareja1: i8 = -1;
    let mut pareja2: i8 = -1;
    let mut is_escalera_mayor = true;
    let mut is_escalera_menor = true;
    for face in 0..face_count.len() {
        let count = face_count[face];
        if count == 5 {
            // repoker
            // return format!("Repoker de {}", get_face_name(face as i8));
            return Hand::Repoker;
        }
        if count == 4 {
            // poker
            // return format!("Poker de {}", get_face_name(face as i8));
            return Hand::Poker;
        }

        if count == 3 {
            trio = face as i8;
        } else if count == 2 {
            if pareja1 > -1 {
                pareja2 = face as i8;
            } else {
                pareja1 = face as i8;
            }
        }

        if (face == 0 && count != 0) || (face != 0 && count != 1) {
            is_escalera_mayor = false;
        } else if (face == 5 && count != 0) || (face != 5 && count != 1) {
            is_escalera_menor = false;
        }
    }

    if trio > -1 {
        if pareja1 > -1 {
            // full
            // return format!(
            //     "Full de {} y {}",
            //     get_face_name(trio as i8),
            //     get_face_name(pareja1 as i8)
            // );
            return Hand::Full;
        } else {
            // trio
            // return format!("Trio de {}", get_face_name(trio as i8));
            return Hand::Trío;
        }
    }

    if pareja1 > -1 {
        if pareja2 > -1 {
            // doble pareja
            // return format!(
            //     "Doble pareja de {} y  {}",
            //     get_face_name(pareja1 as i8),
            //     get_face_name(pareja2 as i8)
            // );
            return Hand::DoblePareja;
        } else {
            // pareja
            // return format!("Pareja de {}", get_face_name(pareja1 as i8));
            return Hand::Pareja;
        }
    }

    if is_escalera_mayor {
        // return "Escalera mayor".to_string();
        return Hand::EscaleraMayor;
    }

    if is_escalera_menor {
        // return "Escalera menor".to_string();
        return Hand::EscaleraMenor;
    }

    return Hand::Violín;
}

fn roll() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=5) as i8
}

fn play(keep_ace: bool) -> HashMap<Hand, u32> {
    let mut map: HashMap<Hand, u32> = HashMap::from([
        (Hand::Violín, 0),
        (Hand::Pareja, 0),
        (Hand::DoblePareja, 0),
        (Hand::Trío, 0),
        (Hand::EscaleraMenor, 0),
        (Hand::EscaleraMayor, 0),
        (Hand::Full, 0),
        (Hand::Poker, 0),
        (Hand::Repoker, 0),
    ]);

    let mut dice: [i8; 5] = [5, 0, 0, 0, 0];
    let mut milestone = 0.0;

    let start = if keep_ace { 1 } else { 0 };
    for i in 0..GAMES {
        for i in start..dice.len() {
            dice[i] = roll();
        }

        let hand = get_hand(dice);
        let count = map.get(&hand).unwrap() + 1;
        map.insert(hand, count);

        let prog = i as f64 / GAMES as f64 * 100.0;
        if prog > milestone {
            print!("\rProgress... {:.2}%", prog);
            milestone += 5.0;
        }
    }

    map
}
