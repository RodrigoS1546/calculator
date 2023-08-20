pub trait TrimInPlace {
    fn trim_in_place(self: &mut Self);
}

impl TrimInPlace for String {
    fn trim_in_place(self: &mut Self) {
        let mut count: usize = 0;
        for c in self.chars().rev() {
            if !c.is_whitespace() {
                break;
            }
            count += 1;
        }
        self.truncate(self.len() - count);
    }
}
