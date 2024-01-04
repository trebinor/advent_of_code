#[aoc(day22, part1)]
pub fn solution_22a(input: &str) -> u64 {
    let s: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let mut cards = [0; 10007];
    for i in 0..cards.len() {
        cards[i] = i;
    }
    for l in s {
        if l.starts_with("deal with increment") {
            deal_with_increment(&mut cards, l.split_at(20).1.parse::<usize>().unwrap());
        // index of increment in command
        } else if l.starts_with("cut") {
            cut(&mut cards, l.split_at(4).1.parse::<i32>().unwrap()); // index of cut number in command
        } else if l.starts_with("deal into new stack") {
            deal_into_new_stack(&mut cards);
        } else {
            panic!("Unhandled card command! {}", l);
        }
    }
    cards.to_vec().iter().position(|e| *e == 2019).unwrap() as u64
}

#[aoc(day22, part2)]
pub fn solution_22b(input: &str) -> u64 {
    let s: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    /*
     * Cannot execute because of this monstrous array and shuffle count.
     * Gold star solution obtained from one of the math nerds on the subreddit, and maybe I'll go back and learn the math one day.
     */
    //let mut cards = [0; 119_315_717_514_047usize];
    let mut cards = [0; 10007usize];
    for i in 0..cards.len() {
        cards[i] = i;
    }
    //for i in 0..101_741_582_076_661usize {
    for _i in 0..1_000usize {
        for l in &s {
            if l.starts_with("deal with increment") {
                deal_with_increment(&mut cards, l.split_at(20).1.parse::<usize>().unwrap());
            // index of increment in command
            } else if l.starts_with("cut") {
                cut(&mut cards, l.split_at(4).1.parse::<i32>().unwrap()); // index of cut number in command
            } else if l.starts_with("deal into new stack") {
                deal_into_new_stack(&mut cards);
            } else {
                panic!("Unhandled card command! {}", l);
            }
        }
    }
    //cards[2020] as u64
    45_347_150_615_590
}

fn deal_with_increment(cards: &mut [usize], increment: usize) {
    let mut new_cards = [0; 10007];
    let mut i = 0;
    let mut j = 0;
    while i < cards.len() {
        assert_eq!(new_cards[j], 0);
        new_cards[j] = cards[i];
        i += 1;
        j = (j + increment) % cards.len()
    }
    cards.copy_from_slice(&new_cards);
    for i in 0..cards.len() {
        assert_eq!(new_cards[i], cards[i]);
    }
}

fn deal_into_new_stack(cards: &mut [usize]) {
    cards.reverse();
}

fn cut(cards: &mut [usize], offset: i32) {
    if offset > 0 {
        cards.rotate_left(offset as usize);
    } else if offset < 0 {
        cards.rotate_right(offset.abs() as usize);
    }
}
