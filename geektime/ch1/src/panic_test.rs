#![allow(unused)]
#[test]
fn main_panic(){
    let earn = pirate_share(100, 0);
    let mut array = [1, 2, 3];
    array.sort();
    let noodles = "noodles".to_string();
    let a = &noodles[..];
    let b = "noodles";

}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}