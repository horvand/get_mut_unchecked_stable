use std::sync::Arc;

pub unsafe fn my_get_mut_unchecked<T: ?Sized>(this: &mut Arc<T>) -> &mut T {
    &mut *(Arc::as_ptr(this) as *mut T)
}

fn main() {
    let mut arc = Arc::new(0);
    let mut arc2 = arc.clone();
    let a = unsafe { my_get_mut_unchecked(&mut arc) };
    let b = unsafe { my_get_mut_unchecked(&mut arc2) };

    *a = 1;
    println!("a = {}, b  = {}", a, b);
    *b = 2;
    println!("a = {}, b  = {}", a, b);
}
