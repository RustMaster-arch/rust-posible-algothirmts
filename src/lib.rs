use std::{thread, time::Duration};

pub fn liniar_search(numbers: &[i32], target: i32) -> bool {
    numbers.iter().find(|number| return **number == target).is_some()
}

pub fn binary_search(numbers: &[u32], target: u32) -> bool {
    let mut min = 0;
    let mut max = numbers.len() - 1;

    while min <= max {
        let mid = (min + max) / 2;
        if numbers[mid] == target {
            return true
        } else if numbers[mid] > target {
            max = mid - 1;
        } else {
            min = mid + 1;
        }
    }
    return false;
}

pub fn two_crystal_balls(breaks: &[bool]) -> u32 {
    let jmp_amount = f32::sqrt(breaks.len() as f32) as usize;
    let mut i = jmp_amount;

    for _ in (0..breaks.len()).step_by(jmp_amount) {
        if breaks[i - 1] == true {
            break;
        }

        i += jmp_amount;
    }

    i -= jmp_amount;
    let mut j = 0;

    while j < jmp_amount && i < breaks.len() {
        if breaks[i] == true {
            return i as u32;
        }

        j += 1;
        i += 1;
    }

    0
}

pub fn two_cystal_balls_esteban(breaks: &[bool]) -> u32 {
    let jump_amount = f32::sqrt(breaks.len() as f32) as usize;

    let mut current_index = 0;
    for jumped_index in (0..breaks.len()).step_by(jump_amount) {
        if breaks[jumped_index] { 
            current_index = jumped_index;
            break;
        }
    };

    let value_to_go = current_index;
    current_index = current_index - jump_amount;

    for index in current_index..value_to_go {
        if breaks[index] { 
            return index as u32
        }
    }

    0
}

pub fn two_crystal_balls_my(breaks: &[bool]) -> u32 {
    for (i, element) in breaks.iter().enumerate() {
        if *element {
            return i as u32
        }
    }

    return 0
}

pub fn two_crystal_balls_my_mid(breaks: &[bool]) -> u32 {
    let min = 0;
    let max = breaks.len() - 1;
    
    let mid = (min + max) / 2;

    if breaks[mid] {
        for i in 0..mid {
            if breaks[i] { return i as u32 }
        }
    }

    if breaks[max] {
        let mid = mid - 1;
        for i in mid..max {
            if breaks[i] { return i as u32 }
        }
    }

    for i in mid..min {
        if breaks[i] { return i as u32 }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::{two_crystal_balls_my, two_crystal_balls_my_mid, two_cystal_balls_esteban};

    #[test]
    fn test_two_crystal_balls_esteban() {
        assert_eq!(3, two_cystal_balls_esteban(&[false, false, false, true, true, true]));
    }

    #[test]
    fn test_two_crystal_balls_mid() {
        assert_eq!(3, two_crystal_balls_my_mid(&[false, false, false, true, true, true]));
    }
}
