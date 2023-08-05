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

    let mut teams = [
        [ ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19), ],
        [ ("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17), ]
    ];
    
    let sorted = teams
    .iter_mut()
    .map(|team|{
        team.sort_by(|&a,&b| a.1.cmp(&b.1).reverse());
        team
    }).collect::<Vec<_>>();
        
    println!("Teams: {:?}", sorted);

    let name_age: Vec<(String, i32)> = vec!(("Jason".to_string(),23),("Somaiya".to_string(),34));
    let okk = name_age.into_iter()
    .map(|(name,age)| name).collect::<Vec<String>>();

    let vector = vec!["Arun","John"];

    let mut it = vector.into_iter();

    loop {
        match it.next() {
            Some(s)=>{
                println!("{:?}",s);
            }
            None=>{
                break;
            }
        }
    }
    //vector.into_iter(|x|)


}