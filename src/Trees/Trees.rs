use std::{rc::Rc,cell::{RefCell, Ref}, collections::VecDeque};
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


impl Tree {

    pub fn new(val:i32)->Self{

        let root = Node{ val,left:None,right:None };
        root.into()
    }

    pub fn level_order(&self){
        let mut queue:VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut result:Vec<i32> = Vec::new();
        
        if let Some(s) = self.root.as_ref() {
            
        queue.push_back(Rc::clone(s));
        while !queue.is_empty() {
            for i in 0..queue.len(){

                if let Some(element) = queue.pop_front() {
                    
                        result.push(element.borrow().val);

                        

                        if let Some(ref okk) = element.borrow().left {
                            queue.push_back(Rc::clone(okk));
                        }

                        if let Some(ref okk) = element.borrow().right {
                            queue.push_back(Rc::clone(okk));
                        }
                    

                    
                }

            }
        }

    }

        print!("{:?}",result);

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

    pub fn delete_node(&self,val:i32){
        Tree::delete_node_recursive(&self.root, val);
    }

    pub fn delete_node_recursive(root:&Option<Rc<RefCell<Node>>>,val:i32)->Option<Rc<RefCell<Node>>>{

                if let Some(s) = root.as_ref() {
                    let temp = s.borrow().val;
                    match temp.cmp(&val) {

                        std::cmp::Ordering::Greater=>{

                            let left =  Tree::delete_node_recursive(&s.borrow().left, val);
                            s.borrow_mut().left = left;
                        }
                        std::cmp::Ordering::Less=>{
                            
                            let right =  Tree::delete_node_recursive(&s.borrow().right, val);
                            s.borrow_mut().right = right;
                        }

                        std::cmp::Ordering::Equal=>{
                            if s.borrow().left.is_none(){
                                return s.borrow().right.clone();
                            }
                            
                            else if s.borrow().right.is_none(){
                                return s.borrow().left.clone();
                            }
                            
                            else{
                                
                                let val = Tree::find_inorder_successor(&s.borrow().right);
                                if let Some(ok) = val {
                                    let final_val = Tree::delete_node_recursive(&s.borrow().right, ok);
                                    s.borrow_mut().val = ok;
                                    s.borrow_mut().right = final_val    
                                }
                                
                                
                                
                            }
                        }
                    }
                }

                return root.clone();
    }

    pub fn find_inorder_successor(node:&Option<Rc<RefCell<Node>>>)->Option<i32>{

        if let Some(s) = node {
            if s.borrow().left.is_none() {
                Some(s.borrow().val)
            }else{
                Tree::find_inorder_successor(&s.borrow().left)
            }
        }else{
            None
        }

    }

    
}

impl From<Node> for Tree {
    fn from(value: Node)->Self{
       Tree{root:Some(Rc::new(RefCell::new(value)))}
    }
}


