pub fn is_leap_year(year: u64) -> bool {
    let d4 = year % 4 == 0;
    let d100 = year % 100 == 0;
    let d400 = year % 400 == 0;
    d4 && !d100 || d400
}
