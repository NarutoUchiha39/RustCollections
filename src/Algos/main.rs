// Question is to find 14 distinct charcters in a string !!

use std::collections::HashSet;

pub fn find_slow(i:&[u8])->usize{

    let ok = i.windows(14)
    .position(|w| {
        return w.iter().collect::<HashSet<_>>().len() == 14;
    }

    ).map(|x| x+14);

    if ok.is_some(){
        return ok.unwrap();
    }

    0
}

pub fn find_faster(i:&[u8])->usize{

    let ok = i.windows(14)
    .position(|w| {
        let mut hash_set:HashSet<&u8> = HashSet::new();

        for i in w{
            if !hash_set.insert(i){
            return false;
            }
        }

        return true;
    }

    ).map(|x| x+14);

    if ok.is_some(){
        return ok.unwrap();
    }

    0
}


pub fn find_faster_vec(i:&[u8])->usize{

    let ok = i.windows(14)
    .position(|w| {
        let mut hash_set:Vec<&u8> = Vec::with_capacity(14);

        for i in w{
            if !hash_set.contains(&i){
                hash_set.push(i);
                
            }
            else {
                return false;
            }
        }
        return true;
    }

    ).map(|x| x+14);

    if ok.is_some(){
        return ok.unwrap();
    }

    0
}

pub fn check_bits(i:&[u8]){

    i.windows(14).position().map()

}

fn main() {
    let input = b"abcdeafghijklmnp"; // Example input string
    let result = find_slow(input);
    println!("Starting index of the first occurrence with 14 distinct characters: {}", result);

    let esult = find_faster(input);
    println!("Starting index of the first occurrence with 14 distinct characters: {}", esult);

    let vesult = find_faster_vec(input);
    println!("Starting index of the first occurrence with 14 distinct characters: {}", vesult);

}
