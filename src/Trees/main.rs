mod Trees;
use Trees::Tree;
pub fn main(){
    
    let tree = Tree::new(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);

    // println!("{:?}",&tree);

    tree.delete_node(4);
    println!("{:?}",&tree);

    tree.level_order();

    
    

}