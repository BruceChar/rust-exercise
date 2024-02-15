// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(PartialEq, Eq, Debug)]
struct Digit([char; 12]);

impl From<&str> for Digit {
    fn from(value: &str) -> Self {
        let s = value.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
        Digit(s.try_into().unwrap())
    }
}

impl ToString for Digit {
    fn to_string(&self) -> String {
        let digits = digits();
        match self {
           d if *d == digits[0] => "0".to_string(),
           d if *d == digits[1] => "1".to_string(),
           d if *d == digits[2] => "2".to_string(),
           d if *d == digits[3] => "3".to_string(),
           d if *d == digits[4] => "4".to_string(),
           d if *d == digits[5] => "5".to_string(),
           d if *d == digits[6] => "6".to_string(),
           d if *d == digits[7] => "7".to_string(),
           d if *d == digits[8] => "8".to_string(),
           d if *d == digits[9] => "9".to_string(),
           _ => "?".to_string()
        }
    }
}

fn digits() -> [Digit; 10] {
    [
        Digit::from(" _ | ||_|   "),
        Digit::from("     |  |   "),
        Digit::from(" _  _||_    "),
        Digit::from(" _  _| _|   "),
        Digit::from("   |_|  |   "),
        Digit::from(" _ |_  _|   "),
        Digit::from(" _ |_ |_|   "),
        Digit::from(" _   |  |   "),
        Digit::from(" _ |_||_|   "),
        Digit::from(" _ |_| _|   "),
    ]
}
pub fn convert(input: &str) -> Result<String, Error> {
    let ocr: Vec<Vec<char>> = input
        .split("\n")
        .to_owned()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if ocr.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(ocr.len()));
    }
    if ocr.iter().any(|x| x.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(
            ocr.iter().map(|x| x.len()).max().unwrap(),
        ));
    }

    // split lines by '\n', every 3 column as a row
    // every 4 rows as a group
    let mut res = String::new();
    for i in (0..ocr.len()).step_by(4) {
        if i > 0 {
            res.push(',');
        }
        for j in (0..ocr[i].len()).step_by(3) {
            let mut group = String::new();
            for k in 0..4 {
                group.push_str(&ocr[i + k][j..j + 3].iter().collect::<String>());
            }
            match digits().iter().find(|x| **x == Digit::from(group.as_ref())) {
                Some(d) => res.push_str(d.to_string().as_ref()),
                None => res.push_str("?"),
            }
        }
    }
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let s = " _ | || |   ";
        let d = Digit::from(s);
        println!("{d:?}");

        let s = " _ \n".to_string() +
        "|_ \n" +
        "|_|\n" +
        "   \n";
        let d = Digit::from(s.as_ref());
        println!("{d:?}");
    }

    #[test]
    fn to_string() {
        let zero = Digit::from(" _ | ||_|   ");
        assert_eq!(zero.to_string(), "0");
        let one = Digit::from("     |  |   ");
        assert_eq!(one.to_string(), "1");
        let two = Digit::from(" _  _||_    ");
        assert_eq!(two.to_string(), "2");
        let three = Digit::from(" _  _| _|   ");
        assert_eq!(three.to_string(), "3");
        let four = Digit::from("   |_|  |   ");
        assert_eq!(four.to_string(), "4");
        let five = Digit::from(" _ |_  _|   ");
        assert_eq!(five.to_string(), "5");
        let six = Digit::from(" _ |_ |_|   ");
        assert_eq!(six.to_string(), "6");
        let seven = Digit::from(" _   |  |   ");
        assert_eq!(seven.to_string(), "7");
        let eight = Digit::from(" _ |_||_|   ");
        assert_eq!(eight.to_string(), "8");
        let nine = Digit::from(" _ |_| _|   ");
        assert_eq!(nine.to_string(), "9");
        let blank = Digit::from("            ");
        assert_eq!(blank.to_string(), "?");
        let garble = Digit::from(" _  _||_ |  ");
        assert_eq!(garble.to_string(), "?");
    }
}
