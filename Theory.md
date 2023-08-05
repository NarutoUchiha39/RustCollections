# Rust Theory

## ***```iterators :```***

1. There are three types of iterators in rust :<br><br>
a )  ***```.iter()```*** <br>
This iterator provides references to the elements being iterated on and are thus immutable. After the function call is over the references go out of scope and thus ```.iter()``` should only be used when we want to read values and not mutate them. Destructuring is required here

Example :
```rust
let vec1 = x.iter()
    .map(|x:&&str| x.len()) //&& required as one & is for string slice and the other is there are .iter() gives reference 
    .fold(0, |acc,len| acc+len);
```
b ) ***```.iter_mut()```*** <br>
This iterator provides mutable references to the variables being iterated on. Useful when you have to sort any vector since sorting requires iteration and mutation both. Destructring is required here
Example :
```rust
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

```
c ) ***```.into_iter()```*** <br>
This iterator takes ownership of the values inside the datastructure being iterated on and thus is useful when we wish to transform elements from one type to the other. No destructuring needed<br> 
Example :
```rust
let name_age: Vec<(String, i32)> = vec!(("Jason".to_string(),23),("Somaiya".to_string(),34));
    let okk = name_age.into_iter()
    .map(|(name,age)| name).collect::<Vec<String>>();

```

### How for loops work internally ? (Rough Idea) <br>

Example :

```rust
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
```

Since for loop internally uses into_iter() using for loop actually moves the value so we should rather use 
```rust
for x in &vector{ // Notice used reference to prevent move of value
    println!("{}",x);
}
```
