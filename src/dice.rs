
use model::Hand;
use rand::Rng;

fn find_biggest_single(face_count: [i8; 6]) -> u32 {
    for face in (0..=face_count.len() - 1).rev() {
        if face_count[face] == 1 {
            return face as u32;
        }
    }
    println!("{:?}", face_count);
    panic!()
}

pub fn get_hand(dice: [i8; 5]) -> (Hand, u32) {
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

pub fn roll() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=5) as i8
}

pub mod model {
    use strum_macros::{Display, EnumIter};

    #[derive(Debug, PartialEq, Eq, Hash, EnumIter, Display)]
    pub enum Hand {
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

    pub struct Result {
        pub hand: Hand,
        pub normal: f64,
        pub ace: f64,
        pub diff: f64,
        pub avg_normal: f64,
        pub avg_ace: f64,
        pub avg_diff: f64,
    }
}
