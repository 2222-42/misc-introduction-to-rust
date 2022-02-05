use std::{
    mem::size_of_val,
    ptr::{read, write},
    slice::from_raw_parts,
};

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

// 特定の型に対して使用した場合にUBになりえるので、unsafeを使う
unsafe fn anything_as_bytes<T: ?Sized>(val: &T) -> &[u8] {
    // unsafe {
    from_raw_parts(val as *const T as *const u8, size_of_val(val))
    // }
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

    unsafe {
        eprintln!("{:x?}", anything_as_bytes(&42));
        // パディングの中身を読み取ろうとするためUB
        eprintln!("{:x?}", anything_as_bytes(&(42, 42.0)));
        let cell = std::cell::Cell::new(42);
        let bytes = anything_as_bytes(&cell);
        // &u8があるのに、横から値が書き換えられてしまうため、UB
        cell.set(84);
        eprintln!("{:x?}", bytes);
    }

    foo(std::ptr::null());
    println!("Hello, world!");
}
