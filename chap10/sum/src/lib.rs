pub fn sum(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        ret += x;
    }
    ret
}

pub fn sum_wrapping(data: &[u8]) -> u8 {
    let mut ret = 0;
    for x in data {
        ret = x.wrapping_add(ret);
    }
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
