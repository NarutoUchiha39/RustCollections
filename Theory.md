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
<br>
<br>
<br>
b ) ***```.iter_mut()```*** <br>
This iterator provides mutable references to the variables being iterated on. Useful when you have to sort any vector since sorting requires iteration and mutation both. Destructring is required here
<br>
<br>
c ) ***```.into_iter()```*** <br>
This iterator takes ownership of the values inside the datastructure being iterated on and thus is useful when we wish to transform elements from one type to the other. No destructuring needed <br> 