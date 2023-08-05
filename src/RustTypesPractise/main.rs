mod Cells;

fn main() {
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let ok = x
        .clone()
        .into_iter()
        .collect::<Vec<_>>();

    println!("{:?}",ok);

    let array = [1,2,3];

    let ok1 = array.iter();
    print!("{:?}",ok1);

    let vec1 = x.iter()
    .map(|x:&&str| x.len())
    .fold(0, |acc,len| acc+len);

}