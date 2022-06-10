fn main() {
    fn test() {
        let v = [1, 2, 3, 4, 5];

        v[99]; // 会出现panic
    }
    test();
}
