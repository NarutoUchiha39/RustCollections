use std::cell::UnsafeCell;

enum state {
    shared(usize),
    exclusive,
    NotShared
}

pub struct RefCell<T>{
    state:state,
    value:UnsafeCell<T>
}

impl<T> RefCell<T> {
    pub fn new(&mut self,value:T)->Self{

        RefCell { state: state::NotShared, value: UnsafeCell::new(value) }
    }

    pub fn borrow(&mut self)->Option<&T>{

        match &self.state {
            state::shared(s) =>{
                self.state = state::shared(s+1);
                unsafe{
                    Some(&*self.value.get())
                }
            },
            state::exclusive =>{
                return None;
            },
            state::NotShared =>{
                self.state = state::shared(1);
                unsafe{
                    Some(&*self.value.get())
                }
            },
        }
    }

    pub fn borrow_mut(&mut self)->Option<&T>{

            match self.state {
                state::exclusive=>{
                    return None;
                }

                _=>{
                    self.state = state::exclusive;
                    unsafe{
                        Some(&mut *self.value.get())
                    }
                }
            }
    }

}