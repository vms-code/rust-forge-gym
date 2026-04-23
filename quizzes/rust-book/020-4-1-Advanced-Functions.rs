fn register1(cb: fn(Event) -> ());
fn register2<F>(cb: F)
where
    F: Fn(Event) -> ();
