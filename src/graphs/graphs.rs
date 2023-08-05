use std::{cell::UnsafeCell, rc::Rc,cell::RefCell};

#[derive(Debug)]
pub(crate) struct Tree{
    root:Option<Rc<RefCell<Node>>>
}
#[derive(Debug)]
pub struct Node{
    val:i32,
    left:Option<Rc<RefCell<Node>>> ,
    right:Option<Rc<RefCell<Node>>>
}
enum state {
    shared(usize),
    exclusive,
    NotShared
}

// pub struct RefCell<T>{
//     state:state,
//     value:UnsafeCell<T>
// }

// impl<T> RefCell<T> {
//     pub fn new(&mut self,value:T)->Self{

//         RefCell { state: state::NotShared, value: UnsafeCell::new(value) }
//     }

//     pub fn borrow(&mut self)->Option<&T>{

//         match &self.state {
//             state::shared(s) =>{
//                 self.state = state::shared(s+1);
//                 unsafe{
//                     Some(&*self.value.get())
//                 }
//             },
//             state::exclusive =>{
//                 return None;
//             },
//             state::NotShared =>{
//                 self.state = state::shared(1);
//                 unsafe{
//                     Some(&*self.value.get())
//                 }
//             },
//         }
//     }

//     pub fn borrow_mut(&mut self)->Option<&T>{

//             match self.state {
//                 state::exclusive=>{
//                     return None;
//                 }

//                 _=>{
//                     self.state = state::exclusive;
//                     unsafe{
//                         Some(&mut *self.value.get())
//                     }
//                 }
//             }
//     }

// }

impl Tree {

    pub fn new(val:i32)->Self{

        let root = Node{ val,left:None,right:None };
        root.into()
    } 

    pub fn insert(&self,val:i32){

        Tree::insert_recursive(&self.root,val);
    }

    pub fn insert_recursive(root:&Option<Rc<RefCell<Node>>>,val:i32){

        if root.is_none(){
            return;
        }

        let root_unwrapped = root.as_ref().unwrap();

        if root_unwrapped.borrow().val > val{
            if !root_unwrapped.borrow().left.is_none(){
                Tree::insert_recursive(&root_unwrapped.borrow().left, val)
            }else {
                root_unwrapped.borrow_mut().left = Some(Rc::new(RefCell::new(Node { val, left: None, right: None })));
            }
        }else if !root_unwrapped.borrow().right.is_none(){
            Tree::insert_recursive(&root_unwrapped.borrow().right, val)
        }else {
            root_unwrapped.borrow_mut().right = Some(Rc::new(RefCell::new(Node { val, left: None, right: None })));
        }
    }

    
}

impl From<Node> for Tree {
    fn from(value: Node)->Self{
       Tree{root:Some(Rc::new(RefCell::new(value)))}
    }
}


