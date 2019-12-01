pub enum Day {
    Day1,
}

pub enum Part {
    One,
    Two,
}

pub struct Config {
    pub day: Day,
    pub part: Part,
    pub input: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let day = match args[1].as_str() {
            "day1" => Day::Day1,
            _ => return Err("unexpected value for 'day' argument"),
        };

        let part = match args[2].as_str() {
            "part1" => Part::One,
            "part2" => Part::Two,
            _ => return Err("unexpected value for 'part' argument"),
        };

        let input = args[3].clone();

        Ok(Config { day, part, input })
    }
}
