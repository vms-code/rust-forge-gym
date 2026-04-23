use std::rc::Rc;
fn main() {
    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        Rc::downgrade(&r2)
    };
    let r5 = Rc::clone(&r1);
    let r6 = r4.upgrade();
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
}
