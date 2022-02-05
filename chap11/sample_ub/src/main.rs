use std::ptr::{read, write};

pub fn foo(ptr: *const i32) {
    if ptr.is_null() {
        eprintln!("ptr is nul");
    } else {
        eprintln!("ptr is not null");
    }
    eprintln!("*ptr is {}", unsafe { *ptr }); // soundness means "unsafeを使わないコードはUBを起こさない"
}

pub unsafe fn replace_with<T, F>(r: &mut T, f: F)
where
    F: FnOnce(&mut T) -> T,
    // T: Default,
{
    let value = f(read(&r));
    write(r, value);
}

unsafe fn unsafe_method(s: &mut String) -> String {
    s.to_string() + " world"
}

fn main() {
    // let mut s = String::from("hello");
    // replace_with(&mut s, |s| s.to_string() + " world");

    // let mut s = String::from("hello");
    // replace_with(&mut s, |_| panic!()); // FIXME: `_`で文字列を解放し、パニックが発生したらもともとの変数`s`の解放処理も発生する。 -> unsafe fn replace_withにする

    let mut s = String::from("hello");
    unsafe {
        replace_with(&mut s, |s| unsafe_method(s));
    }

    foo(std::ptr::null());
    println!("Hello, world!");
}
