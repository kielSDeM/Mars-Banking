
use std::cell::RefCell;
use std::rc::{Rc,Weak};
//cycle References
//Arc and Mutex for multithread
#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbNode<T>{
    data:T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,

}

impl<T> DbList<T>{
    pub fn new()->Self{
        DbList{
            last:None,
            first:None,
        }
    }

    pub fn push_front(&mut self, x: &'static str, x0: &'static str, x1: f64, x2: &'static str, data:T){
        match self.first.take() {
            Some(r)=>{
                let new_front = Rc::new(RefCell::new(DbNode{
                    data,
                    next:Some(r.clone()),
                    prev:None,
                }));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None=>{
                let new_data = Rc::new(RefCell::new(DbNode{
                    data,
                    next:None,
                    prev:None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            },
        }
    }
    pub fn push_back(&mut self,data:T){
        match self.last.take() {
            Some(r)=>{
                let new_back = Rc::new(RefCell::new(DbNode{
                    data,
                    prev:Some(r.clone()),
                    next:None,
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None=>{
                let new_data = Rc::new(RefCell::new(DbNode{
                    data,
                    next:None,
                    prev:None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            },
        }
    }
}
fn tran_history() {
    let mut dl = DbList::new();
    dl.push_front("Amazon", "$", 46.75, "date", 4);
    dl.push_back(11);
    dl.push_front("Amazon", "$",20.00,"date", 5);
    dl.push_front("BestBuy","$", 10.00, "date", 4);
    dl.push_front("LegoStore","$", 150.00, "date", 43);
    dl.push_back(15);

    println!("dl {:?}", dl);
}