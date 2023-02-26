
use compound_duration::format_dhms;
use num_format::{Locale, ToFormattedString};
use std::thread;
use std::{collections::HashMap, time::Instant};
use strum::IntoEnumIterator;

use crate::dice::model::{Hand, Result};
use crate::dice::{get_hand, roll};
use crate::{GAMES, THREADS};

pub fn simulation() {
    let start = Instant::now();
    let normal_play = play_multi_thread(false);
    let ace_play = play_multi_thread(true);

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

    println!();
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

fn play_multi_thread(keep_ace: bool) -> Vec<HashMap<Hand, (u64, i128)>> {
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
