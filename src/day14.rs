use std::collections::HashMap;
use std::fs;

pub fn run_part1() -> u64 {
    part1(&input(&read_file()))
}

pub fn run_part2() -> u64 {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("data/day14.txt").expect("Error reading the file")
}

#[derive(PartialEq, Eq, Hash)]
struct Chemical<'a> {
    name: &'a str,
    amount: u64,
}

impl<'a> Chemical<'a> {
    fn times(self: &Self, n: u64) -> Self {
        Chemical {
            name: self.name,
            amount: self.amount * n,
        }
    }
}

type Reactions<'a> = HashMap<Chemical<'a>, Vec<Chemical<'a>>>;

fn input<'a>(data: &'a String) -> Reactions<'a> {
    data.trim()
        .lines()
        .map(|line| {
            let parse = |s: &'a str| {
                let p = s.trim().split(" ").collect::<Vec<_>>();
                Chemical {
                    name: p[1],
                    amount: p[0].parse().unwrap(),
                }
            };
            let parts = line.split("=>").collect::<Vec<_>>();
            let from = parts[0].split(",").map(parse).collect::<Vec<_>>();
            let to = parse(parts[1]);
            (to, from)
        })
        .collect()
}

fn part1(reactions: &Reactions) -> u64 {
    collect_fuel(reactions, 1)
}

fn part2(reactions: &Reactions) -> u64 {
    const MAX_ORE: u64 = 1_000_000_000_000;

    let mut low = std::u64::MIN;
    let mut high = MAX_ORE;

    loop {
        if high - low <= 1 {
            return low;
        }

        let mid = (low + high) / 2;
        let fuel = collect_fuel(reactions, mid);

        if fuel < MAX_ORE {
            low = mid;
        } else if fuel > MAX_ORE {
            high = mid;
        }
    }
}

fn collect_fuel(reactions: &Reactions, fuel_count: u64) -> u64 {
    scan(
        reactions,
        &Chemical {
            name: "FUEL",
            amount: fuel_count,
        },
        &mut HashMap::new(),
    )
}

fn scan(reactions: &Reactions, desired: &Chemical, surplus_bag: &mut HashMap<String, u64>) -> u64 {
    if desired.name == "ORE" {
        return desired.amount;
    }

    let surplus = surplus_bag.entry(desired.name.to_owned()).or_insert(0);
    if *surplus >= desired.amount {
        *surplus -= desired.amount;
        return 0;
    }

    let need = desired.amount - *surplus;
    *surplus = 0;

    let (reaction, ingredients) = reactions
        .iter()
        .find(|(r, _)| r.name == desired.name)
        .unwrap();

    let need_reactions = ((need as f64) / (reaction.amount as f64)).ceil() as u64;
    let new_surplus = reaction.amount * need_reactions - need;

    *surplus_bag.entry(desired.name.to_owned()).or_default() += new_surplus;

    ingredients
        .iter()
        .map(|ingredient| scan(reactions, &ingredient.times(need_reactions), surplus_bag))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            31,
            part1(&input(
                &"
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
"
                .to_owned()
            ))
        );

        assert_eq!(
            165,
            part1(&input(
                &"
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
"
                .to_owned()
            ))
        );

        assert_eq!(
            13312,
            part1(&input(
                &"
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
"
                .to_owned()
            ))
        );

        assert_eq!(
            180697,
            part1(&input(
                &"
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
"
                .to_owned()
            ))
        );

        assert_eq!(
            2210736,
            part1(&input(
                &"
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"
                .to_owned()
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            82892753,
            part2(&input(
                &"
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
"
                .to_owned()
            ))
        );

        assert_eq!(
            5586022,
            part2(&input(
                &"
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
"
                .to_owned()
            ))
        );

        assert_eq!(
            460664,
            part2(&input(
                &"
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"
                .to_owned()
            ))
        );
    }
}
