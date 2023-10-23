pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }
    let last = format!("And all for the want of a {}.", list[0]);
    let mut v = vec![];
    for s in list.windows(2) {
        v.push(format!("For want of a {} the {} was lost.", s[0], s[1]));
    }
    v.push(last);
    v.join("\n")
}
