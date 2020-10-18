use regex::Regex;

const ALPHABET: &'static [&'static str] = &[
    "A", "B", "C", "D", "E",
    "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O",
    "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y",
    "Z",
];

pub fn column_index_from_string<S: Into<String>>(column:S)->usize {
    let column_c = column.into().clone();
    match column_c.len() {
        3 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string()) * 676;
            result += get_index(&column_c.chars().nth(1).unwrap().to_string()) * 26;
            result += get_index(&column_c.chars().nth(2).unwrap().to_string());
            return result;
        },
        2 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string()) * 26;
            result += get_index(&column_c.chars().nth(1).unwrap().to_string());
            return result;
        },
        1 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string());
            return result;
        },
        _ => {
            panic!("longer than 3 characters");
        }
    }
}

fn get_index(column:&str)->usize {
    let mut i = 0;
    for tar in self::ALPHABET {
        if tar == &column {
            return i;
        }
        i += 1;
    }
    panic!("illegal character");
}

pub fn string_from_column_index(column_index:&usize)->String {
    if column_index < &1usize {
        panic!("Column number starts from 1.");
    }
    let mut result: String = String::from("");
    let mut index_value = column_index.clone();
    while index_value > 0 {
        let character_value = index_value % 26;
        index_value = (index_value - character_value) / 26;
        result = format!("{}{}", self::ALPHABET.get(character_value - 1).unwrap(), result);
    }
    result
}

pub fn coordinate_from_string(coordinate:&str)->Vec<&str> {
    let re = Regex::new(r"[A-Z]+").unwrap();
    let caps = re.captures(coordinate).unwrap();
    let col = caps.get(0).unwrap().as_str();

    let re = Regex::new(r"[0-9]+").unwrap();
    let caps = re.captures(coordinate).unwrap();
    let row = caps.get(0).unwrap().as_str();

    vec![col, row]
}

pub fn coordinate_from_index(col:&usize, row:&usize)->String {
    format!(
        "{}{}",
        string_from_column_index(&col),
        row
    )
}

pub fn index_from_coordinate<S: Into<String>>(coordinate:S)->Vec<usize> {
    let con = coordinate.into();
    let split = coordinate_from_string(con.as_str());
    let col = column_index_from_string(split[0]);
    let row = split[1].parse::<usize>().unwrap();
    vec![col, row]
}
