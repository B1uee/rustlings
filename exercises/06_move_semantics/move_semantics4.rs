fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;  // 同一时间只有一个可变引用
        y.push(42);
        let z = &mut x; // 此时y就不能用了
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
