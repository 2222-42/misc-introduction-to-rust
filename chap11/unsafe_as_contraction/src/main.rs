/// ### 事前条件
/// sliceの要素は昇順であるとする。
/// ### 事後条件
/// 得られるusizeで、それ未満のindexの要素は`target`未満
/// 得られるusizeで、それ以上のindexの要素は`target`以上
pub fn lower_bound(slice: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = slice.len();

    // ループ不変条件:
    // - left <= right
    // - slice[..left]の任意の要素 < target
    // - slice[right..]の任意の要素 >= target
    // ループ変量: right - leftが0以上で、狭義単調現象する

    while left < right {
        let mid = left + (right - left) / 2; // NOTICE: オーバーフローに注意
        if slice[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

/// 矩形。`left < x < right` `top < y < bottom`となる領域。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

// 空の矩形を同一視したい。　→ ふぃーるどのpubを取り除き、カプセル化する。
impl Rect {
    pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        // 型の不変条件を与える。空なら、left=right=top=bottom=0である。
        if right < left || bottom < top {
            Rect {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            }
        } else {
            Rect {
                left,
                right,
                top,
                bottom,
            }
        }
    }
}

pub fn split_at_dots(s: &str) -> (&str, &str) {
    if let Some(dot) = s.find('.') {
        // (&s[..dot], &s[dot + 1..])

        // get_uncheckedは、s[..dot]の要素数がs.len()以下でなければならない。
        // 1. dotはfindの返り値 -> 0<=dot<s.len() && dotはUTF-8境界である。
        // 2. dotはfindの返り値 -> 0<=dot<s.len()-1 && s[dot]の直後は'.'であり、UTF-8境界である。
        unsafe { (s.get_unchecked(..dot), s.get_unchecked(dot + 1..)) }
    } else {
        (s, "")
    }
}

fn main() {
    let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", lower_bound(slice, 3));
}
