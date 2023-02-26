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
    let orphan_play = play_multi_thread(true);

    let mut results: Vec<Result> = Vec::new();

    for hand in Hand::iter() {
        let mut count_normal: u64 = 0;
        let mut count_ace: u64 = 0;
        for result in normal_play.iter() {
            let c = result.get(&hand).unwrap();
            count_normal = count_normal + c;
        }
        for result in orphan_play.iter() {
            let c = result.get(&hand).unwrap();
            count_ace = count_ace + c;
        }

        let p_normal = count_normal as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_ace = count_ace as f64 / (GAMES * THREADS) as f64 * 100.0;
        let p_diff = p_ace - p_normal;

        results.push(Result {
            hand,
            normal: p_normal,
            ace: p_ace,
            diff: p_diff,
            avg_normal: 0.0,
            avg_ace: 0.0,
            avg_diff: 0.0,
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
    print!("                        ");
    for r in &results {
        print!(" | {0: <15}", r.hand)
    }

    print!(
        "\n-------------------------------------------------------------------------------------"
    );
    println!(
        "-------------------------------------------------------------------------------------"
    );

    print!("trio                    ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}%", r.normal))
    }
    print!("\nhuérfanos               ");
    for r in &results {
        print!(" | {0: <15}", format!("{:.4}%", r.ace))
    }
    print!("\ndiff                    ");
    for r in &results {
        if r.diff > 0.0 {
            print!(" | +{0: <14}", format!("{:.4}", r.diff))
        } else {
            print!(" | {0: <15}", format!("{:.4}", r.diff))
        }
    }

    println!("\n\n\n\n");
}

fn play_multi_thread(play_orphans: bool) -> Vec<HashMap<Hand, u64>> {
    let mut handles = Vec::new();
    let mut results = Vec::new();

    for i in 0..THREADS {
        let h = thread::spawn(move || play(play_orphans, format!("{}", i)));
        handles.push(h);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}

fn play(play_orphans: bool, thread: String) -> HashMap<Hand, u64> {
    let mut map: HashMap<Hand, u64> = HashMap::from([
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

    let mut dice: [i8; 5] = [3, 3, 4, 4, 5];
    let mut milestone = 0.0;

    let start = if play_orphans { dice.len() - 1 } else { 2 };
    for i in 0..GAMES {
        for i in start..dice.len() {
            dice[i] = roll();
        }

        let (hand, _value) = get_hand(dice);

        let prev_count = map.get(&hand).unwrap();

        map.insert(hand, prev_count + 1);

        let prog = i as f64 / GAMES as f64 * 100.0;
        if prog > milestone {
            println!("T{} progress... {:.2}%", thread, prog);
            milestone += 25.0;
        }
    }

    map
}
