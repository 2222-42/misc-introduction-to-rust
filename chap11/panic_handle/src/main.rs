use std::panic::{set_hook, take_hook};

fn main() {
    let hook_origin = take_hook();
    set_hook(Box::new(move |info| {
        eprintln!("panic occured!");
        hook_origin(info);
    }));
    assert!(false);
}
