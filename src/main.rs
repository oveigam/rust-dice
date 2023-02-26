use compound_duration::format_dhms;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::thread;
use std::{collections::HashMap, time::Instant};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

static PRINT_TOTAL_VALUES: bool = false;

static GAMES: u64 = 10000000000;
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

struct Result {
    hand: Hand,
    normal: f64,
    ace: f64,
    diff: f64,
    value_normal: i128,
    value_ace: i128,
    value_diff: i128,
    avg_normal: f64,
    avg_ace: f64,
    avg_diff: f64,
}

fn main() {
    let start = Instant::now();
    let normal_play = simulation(false);
    let ace_play = simulation(true);

    let mut results: Vec<Result> = Vec::new();

    for hand in Hand::iter() {
        let mut count_normal: u64 = 0;
        let mut count_ace: u64 = 0;
        let mut value_normal: i128 = 0;
        let mut value_ace: i128 = 0;
        for result in normal_play.iter() {
            let (c, v) = result.get(&hand).unwrap();
            count_normal = count_normal + c;
            value_normal = value_normal + v;
        }
        for result in ace_play.iter() {
            let (c, v) = result.get(&hand).unwrap();
            count_ace = count_ace + c;
            value_ace = value_ace + v;
        }

        let p_normal = count_normal as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_ace = count_ace as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_diff = p_ace - p_normal;

        let avg_normal = value_normal as f64 / count_normal as f64;
        let avg_ace = value_ace as f64 / count_ace as f64;
        let avg_diff = avg_ace - avg_normal;

        results.push(Result {
            hand,
            normal: p_normal,
            ace: p_ace,
            diff: p_diff,
            value_normal,
            value_ace,
            value_diff: value_ace - value_normal,
            avg_normal,
            avg_ace,
            avg_diff,
        });
    }

    println!("\n\n\n\n");
    println!(
        "Partidas simuladas:  {}",
        (GAMES * THREADS).to_formatted_string(&Locale::fr)
    );
    println!(
        "Tiempo de ejecución: {:?}",
        format_dhms(start.elapsed().as_secs())
    );

    print!("                      ");
    for r in &results {
        print!(" | {0: <15}", r.hand)
    }

    print!(
        "\n-------------------------------------------------------------------------------------"
    );
    println!(
        "-------------------------------------------------------------------------------------"
    );

    print!("normal                ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}%", r.normal))
    }
    print!("\nguardar               ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}%", r.ace))
    }
    print!("\ndiff                  ");
    for r in &results {
        if r.diff > 0.0 {
            print!(" | +{0: <14}", format!("{:.4}", r.diff))
        } else {
            print!(" | {0: <15}", format!("{:.4}", r.diff))
        }
    }

    if PRINT_TOTAL_VALUES {
        print!(
        "\n-------------------------------------------------------------------------------------"
    );
        println!(
            "-------------------------------------------------------------------------------------"
        );

        print!("total value normal    ");
        for r in &results {
            print!(" | {0: <15}", format!("{}", r.value_normal))
        }
        print!("\ntotal value guardar   ");
        for r in &results {
            print!(" | {0: <15}", format!("{}", r.value_ace))
        }
        print!("\ntotal value diff      ");
        for r in &results {
            if r.value_diff > 0 {
                print!(" | +{0: <14}", format!("{:.4}", r.value_diff))
            } else {
                print!(" | {0: <15}", format!("{:.4}", r.value_diff))
            }
        }
    }

    print!(
        "\n-------------------------------------------------------------------------------------"
    );
    println!(
        "-------------------------------------------------------------------------------------"
    );

    print!("min/max avg           ");
    print!(" | {: <15}", format!("{}/{}", 0, 0));
    // Pareja
    print!(" | {: <15}", format!("{}/{}", 1 * 20 + 2, 6 * 20 + 5));
    // Doble pareja
    print!(
        " | {: <15}",
        format!("{}/{}", 1 * 20 + 2 * 20 + 3, 6 * 20 + 5 * 20 + 4)
    );
    // trio
    print!(" | {: <15}", format!("{}/{}", 1 * 300 + 2, 6 * 300 + 5));
    //escaleras
    print!(" | {: <15}", format!("{}/{}", 1, 1));
    print!(" | {: <15}", format!("{}/{}", 1, 1));
    // full
    print!(
        " | {: <15}",
        format!("{}/{}", 1 * 300 + 2 * 20, 6 * 300 + 5 * 20)
    );
    // poker
    print!(" | {: <15}", format!("{}/{}", 1 * 4000 + 2, 6 * 4000 + 5));
    // repoker
    print!(" | {: <15}", format!("{}/{}", 1 * 50000, 6 * 50000));

    print!(
        "\n-------------------------------------------------------------------------------------"
    );
    println!(
        "-------------------------------------------------------------------------------------"
    );

    print!("avg value normal      ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}", r.avg_normal))
    }
    print!("\navg value guardar     ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}", r.avg_ace))
    }
    print!("\navg value diff        ");
    for r in &results {
        if r.avg_diff > 0.0 {
            print!(" | +{0: <14}", format!("{:.4}", r.avg_diff))
        } else {
            print!(" | {0: <15}", format!("{:.4}", r.avg_diff))
        }
    }

    println!("\n\n\n\n");
}

fn simulation(keep_ace: bool) -> Vec<HashMap<Hand, (u64, i128)>> {
    let mut handles = Vec::new();
    let mut results = Vec::new();

    for i in 0..THREADS {
        let h = thread::spawn(move || play(keep_ace, format!("{}", i)));
        handles.push(h);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}

fn find_biggest_single(face_count: [i8; 6]) -> u32 {
    for face in (0..=face_count.len() - 1).rev() {
        if face_count[face] == 1 {
            return face as u32;
        }
    }
    println!("{:?}", face_count);
    panic!()
}

fn get_hand(dice: [i8; 5]) -> (Hand, u32) {
    let mut face_count: [i8; 6] = [0, 0, 0, 0, 0, 0]; // index represents the face

    for face in dice {
        face_count[face as usize] = face_count[face as usize] + 1;
    }

    let mut repoker: i8 = -1;
    let mut poker: i8 = -1;
    let mut trio: i8 = -1;
    let mut pareja1: i8 = -1;
    let mut pareja2: i8 = -1;
    let mut is_escalera_mayor = true;
    let mut is_escalera_menor = true;
    for face in 0..face_count.len() {
        let count = face_count[face];
        if count == 5 {
            // repoker
            repoker = face as i8;
        }
        if count == 4 {
            // poker
            poker = face as i8;
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

    if repoker > -1 {
        let repoker_value = (repoker as u32 + 1) * 50000;
        return (Hand::Repoker, repoker_value);
    }

    if poker > -1 {
        let poker_value = (poker as u32 + 1) * 4000;
        let single_value = find_biggest_single(face_count) + 1;
        return (Hand::Poker, poker_value + single_value);
    }

    if trio > -1 {
        let trio_value = (trio as u32 + 1) * 300;
        if pareja1 > -1 {
            let pareja_value = (pareja1 as u32 + 1) * 20;
            return (Hand::Full, trio_value + pareja_value);
        } else {
            // trio
            let single_value = find_biggest_single(face_count) + 1;
            return (Hand::Trío, trio_value + single_value);
        }
    }

    if pareja1 > -1 {
        if pareja2 > -1 {
            // doble pareja
            let pareja1_value = (pareja1 as u32 + 1) * 20;
            let pareja2_value = (pareja2 as u32 + 1) * 20;
            let single_value = find_biggest_single(face_count) + 1;
            return (
                Hand::DoblePareja,
                pareja1_value + pareja2_value + single_value,
            );
        } else {
            // pareja
            let pareja_value = (pareja1 as u32 + 1) * 20;
            let single_value = find_biggest_single(face_count) + 1;
            return (Hand::Pareja, pareja_value + single_value);
        }
    }

    if is_escalera_mayor {
        // escalera mayor
        return (Hand::EscaleraMayor, 1);
    }

    if is_escalera_menor {
        // escalera menor
        return (Hand::EscaleraMenor, 1);
    }

    // escalera de violin
    return (Hand::Violín, 0);
}

fn roll() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=5) as i8
}

fn play(keep_ace: bool, thread: String) -> HashMap<Hand, (u64, i128)> {
    let mut map: HashMap<Hand, (u64, i128)> = HashMap::from([
        (Hand::Violín, (0, 0)),
        (Hand::Pareja, (0, 0)),
        (Hand::DoblePareja, (0, 0)),
        (Hand::Trío, (0, 0)),
        (Hand::EscaleraMenor, (0, 0)),
        (Hand::EscaleraMayor, (0, 0)),
        (Hand::Full, (0, 0)),
        (Hand::Poker, (0, 0)),
        (Hand::Repoker, (0, 0)),
    ]);

    let mut dice: [i8; 5] = [5, 5, 5, 5, 5];
    let mut milestone = 0.0;

    let start = if keep_ace { 1 } else { 0 };
    for i in 0..GAMES {
        for i in start..dice.len() {
            dice[i] = roll();
        }

        let (hand, value) = get_hand(dice);

        let (prev_count, prev_value) = map.get(&hand).unwrap();

        map.insert(hand, (prev_count + 1, prev_value + value as i128));

        let prog = i as f64 / GAMES as f64 * 100.0;
        if prog > milestone {
            println!("T{} progress... {:.2}%", thread, prog);
            milestone += 25.0;
        }
    }

    map
}
