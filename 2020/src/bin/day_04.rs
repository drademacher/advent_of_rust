use regex::Regex;
use std::collections::HashMap;

fn main() {
    let raw_input = include_str!("../../resources/day_4.txt");
    println!(
        "First part: {}\nSecond part: {}",
        part_one(read_input_file(raw_input)),
        part_two(read_input_file(raw_input))
    );
}

type Passport = HashMap<String, String>;

fn read_input_file(raw_input: &str) -> Vec<Passport> {
    let lines: Vec<String> = format_file(raw_input);

    let mut result = Vec::new();

    for line in lines {
        let mut passport = Passport::new();

        let entries = line.split(&" ");
        for entry in entries {
            let entry_split = entry.split(":").collect::<Vec<_>>();
            passport.insert(entry_split[0].to_string(), entry_split[1].to_string());
        }
        result.push(passport);
    }

    return result;
}

fn format_file(raw_input: &str) -> Vec<String> {
    let re_single_line_break = Regex::new(r"(\S)\n(\S)").unwrap();
    let intermediate_result = re_single_line_break.replace_all(raw_input, "${1} ${2}");

    let re_dobule_line_break = Regex::new(r"\n\n").unwrap();
    let result = re_dobule_line_break.replace_all(&intermediate_result, "\n");

    return result
        .lines()
        .map(|s| String::from(s))
        .filter(|s| s != "")
        .collect::<Vec<_>>();
}

fn part_one(passports: Vec<Passport>) -> usize {
    let mandatory_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut result = 0;

    for passport in passports {
        let keys = passport.iter().map(|(k, _)| k).collect::<Vec<_>>();

        let passport_has_all_mandatory_keys = mandatory_keys.iter().all(|x| keys.contains(&x));

        if passport_has_all_mandatory_keys {
            result += 1;
        }
    }

    return result;
}

fn part_two(passports: Vec<Passport>) -> usize {
    let mut result = 0;

    for passport in passports {
        if let Some(byr) = passport.get("byr") {
            let byr_num = byr.parse::<usize>().unwrap();
            if byr_num < 1920 || byr_num > 2002 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(iyr) = passport.get("iyr") {
            let iyr_num = iyr.parse::<usize>().unwrap();
            if iyr_num < 2010 || iyr_num > 2020 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(eyr) = passport.get("eyr") {
            let eyr_num = eyr.parse::<usize>().unwrap();
            if eyr_num < 2020 || eyr_num > 2030 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(hgt) = passport.get("hgt") {
            let regex = Regex::new(r"^(\d+)(.+)$").unwrap();
            let capture = regex.captures_iter(hgt).next().unwrap();

            let number = capture[1].parse::<usize>().unwrap();
            let unit = capture.get(2).unwrap().as_str().trim();

            if unit == "cm" {
                if number < 150 || number > 193 {
                    continue;
                }
            } else if unit == "in" {
                if number < 59 || number > 76 {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }

        if let Some(hcl) = passport.get("hcl") {
            let regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            if !regex.is_match(hcl) {
                continue;
            }
        } else {
            continue;
        }

        if let Some(ecl) = passport.get("ecl") {
            let regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
            if !regex.is_match(ecl) {
                continue;
            }
        } else {
            continue;
        }

        if let Some(pid) = passport.get("pid") {
            let regex = Regex::new(r"^\d{9}$").unwrap();
            if !regex.is_match(pid) {
                continue;
            }
        } else {
            continue;
        }

        result += 1;
    }

    return result;
}

#[test]
fn example_passports() {
    let test_input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    assert_eq!(part_one(read_input_file(test_input)), 2);
}

#[test]
fn example_passports_validation() {
    let test_input = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

    assert_eq!(part_two(read_input_file(test_input)), 4);
}
