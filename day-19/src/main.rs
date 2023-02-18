use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";
const PART1_TIME: usize = 24;

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";
const PART2_TIME: usize = 32;

#[derive(Debug, Clone, Copy)]
struct ASSETS {
    ore: usize,
    clay: usize,
    obsdn: usize,
    geode: usize,
}

#[derive(Debug, Clone, Copy)]
struct ROBOTS {
    ore: usize,
    clay: usize,
    obsdn: usize,
    geode: usize,
}

struct BLUEPRINT {
    ore_ore: usize,
    clay_ore: usize,
    obsdn_ore: usize,
    obsdn_clay: usize,
    geode_ore: usize,
    geode_obsdn: usize,
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn cap_to_usize(caps: &regex::Captures, name: &str) -> usize {
    caps.name(name).unwrap().as_str().parse::<usize>().unwrap()
}

fn parse_line(line: &str) -> BLUEPRINT {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Blueprint \d+: Each ore robot costs (?P<v1>\d+) ore. Each clay robot costs (?P<v2>\d+) ore. Each obsidian robot costs (?P<v3>\d+) ore and (?P<v4>\d+) clay. Each geode robot costs (?P<v5>\d+) ore and (?P<v6>\d+) obsidian.$"
        )
        .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    BLUEPRINT {
        ore_ore: cap_to_usize(&caps, "v1"),
        clay_ore: cap_to_usize(&caps, "v2"),
        obsdn_ore: cap_to_usize(&caps, "v3"),
        obsdn_clay: cap_to_usize(&caps, "v4"),
        geode_ore: cap_to_usize(&caps, "v5"),
        geode_obsdn: cap_to_usize(&caps, "v6"),
    }
}

fn time_to_build(asset_have: usize, asset_need: usize, robot: usize) -> usize {
    if asset_have >= asset_need {
        0
    } else {
        ((asset_need - asset_have) + robot - 1) / robot
    }
}

fn branch_n_bound(
    blueprint: &BLUEPRINT,
    max_time: usize,
    t: usize,
    assets: &mut ASSETS,
    robots: &mut ROBOTS,
    res: &mut usize,
) -> bool {
    let mut o_assets = assets.clone();
    let mut o_robots = robots.clone();
    for _ in t + 1..max_time {
        let buildable_robots = ROBOTS {
            ore: o_assets.ore / blueprint.ore_ore,
            clay: o_assets.ore / blueprint.clay_ore,
            obsdn: std::cmp::min(
                o_assets.ore / blueprint.obsdn_ore,
                o_assets.clay / blueprint.obsdn_clay,
            ),
            geode: std::cmp::min(
                o_assets.ore / blueprint.geode_ore,
                o_assets.obsdn / blueprint.geode_obsdn,
            ),
        };
        o_assets.ore += o_robots.ore;
        o_assets.clay += o_robots.clay;
        o_assets.obsdn += o_robots.obsdn;
        o_assets.geode += o_robots.geode;
        if o_assets.geode > *res {
            return false;
        }
        o_robots.ore += buildable_robots.ore;
        o_robots.clay += buildable_robots.clay;
        o_robots.obsdn += buildable_robots.obsdn;
        o_robots.geode += buildable_robots.geode;
    }
    o_assets.geode += o_robots.geode;
    o_assets.geode <= *res
}

fn search(
    blueprint: &BLUEPRINT,
    max_time: usize,
    t: usize,
    assets: &mut ASSETS,
    robots: &mut ROBOTS,
    res: &mut usize,
) {
    if branch_n_bound(blueprint, max_time, t, assets, robots, res) {
        return;
    }
    // do nothing
    {
        let sub_res = (max_time - t) * robots.geode + assets.geode;
        if sub_res > *res {
            *res = sub_res;
        }
    }
    // next build
    {
        // geode robot
        if robots.obsdn > 0 {
            let dt = std::cmp::max(
                time_to_build(assets.ore, blueprint.geode_ore, robots.ore),
                time_to_build(assets.obsdn, blueprint.geode_obsdn, robots.obsdn),
            );
            if t + dt + 1 < max_time {
                assets.ore += (dt + 1) * robots.ore;
                assets.clay += (dt + 1) * robots.clay;
                assets.obsdn += (dt + 1) * robots.obsdn;
                assets.geode += (dt + 1) * robots.geode;

                assets.ore -= blueprint.geode_ore;
                assets.obsdn -= blueprint.geode_obsdn;
                robots.geode += 1;
                search(blueprint, max_time, t + dt + 1, assets, robots, res);
                assets.ore += blueprint.geode_ore;
                assets.obsdn += blueprint.geode_obsdn;
                robots.geode -= 1;

                assets.ore -= (dt + 1) * robots.ore;
                assets.clay -= (dt + 1) * robots.clay;
                assets.obsdn -= (dt + 1) * robots.obsdn;
                assets.geode -= (dt + 1) * robots.geode;
            }
        }
        // obsidian robot
        if robots.clay > 0 {
            let dt = std::cmp::max(
                time_to_build(assets.ore, blueprint.obsdn_ore, robots.ore),
                time_to_build(assets.clay, blueprint.obsdn_clay, robots.clay),
            );
            if t + dt + 1 < max_time {
                assets.ore += (dt + 1) * robots.ore;
                assets.clay += (dt + 1) * robots.clay;
                assets.obsdn += (dt + 1) * robots.obsdn;
                assets.geode += (dt + 1) * robots.geode;

                assets.ore -= blueprint.obsdn_ore;
                assets.clay -= blueprint.obsdn_clay;
                robots.obsdn += 1;
                search(blueprint, max_time, t + dt + 1, assets, robots, res);
                assets.ore += blueprint.obsdn_ore;
                assets.clay += blueprint.obsdn_clay;
                robots.obsdn -= 1;

                assets.ore -= (dt + 1) * robots.ore;
                assets.clay -= (dt + 1) * robots.clay;
                assets.obsdn -= (dt + 1) * robots.obsdn;
                assets.geode -= (dt + 1) * robots.geode;
            }
        }
        // clay robot
        let dt = time_to_build(assets.ore, blueprint.clay_ore, robots.ore);
        if t + dt + 1 < max_time {
            assets.ore += (dt + 1) * robots.ore;
            assets.clay += (dt + 1) * robots.clay;
            assets.obsdn += (dt + 1) * robots.obsdn;
            assets.geode += (dt + 1) * robots.geode;

            assets.ore -= blueprint.clay_ore;
            robots.clay += 1;
            search(blueprint, max_time, t + dt + 1, assets, robots, res);
            assets.ore += blueprint.clay_ore;
            robots.clay -= 1;

            assets.ore -= (dt + 1) * robots.ore;
            assets.clay -= (dt + 1) * robots.clay;
            assets.obsdn -= (dt + 1) * robots.obsdn;
            assets.geode -= (dt + 1) * robots.geode;
        }
        // ore robot
        let dt = time_to_build(assets.ore, blueprint.ore_ore, robots.ore);
        if t + dt + 1 < max_time {
            assets.ore += (dt + 1) * robots.ore;
            assets.clay += (dt + 1) * robots.clay;
            assets.obsdn += (dt + 1) * robots.obsdn;
            assets.geode += (dt + 1) * robots.geode;

            assets.ore -= blueprint.ore_ore;
            robots.ore += 1;
            search(blueprint, max_time, t + dt + 1, assets, robots, res);
            assets.ore += blueprint.ore_ore;
            robots.ore -= 1;

            assets.ore -= (dt + 1) * robots.ore;
            assets.clay -= (dt + 1) * robots.clay;
            assets.obsdn -= (dt + 1) * robots.obsdn;
            assets.geode -= (dt + 1) * robots.geode;
        }
    }
}

fn f(blueprint: &BLUEPRINT, max_time: usize) -> usize {
    let mut assets = ASSETS {
        ore: 1,
        clay: 0,
        obsdn: 0,
        geode: 0,
    };
    let mut robots = ROBOTS {
        ore: 1,
        clay: 0,
        obsdn: 0,
        geode: 0,
    };
    let mut res = 0usize;
    search(blueprint, max_time, 1, &mut assets, &mut robots, &mut res);
    res
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let mut res = 0;
    for (idx, blueprint) in input.split("\n").map(|line| parse_line(line)).enumerate() {
        let sub_res = f(&blueprint, PART1_TIME);
        // println!("{} {}", idx + 1, sub_res);
        res += (idx + 1) * sub_res;
    }
    println!("{}", res);
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let mut res = 1;
    for (idx, blueprint) in input.split("\n").map(|line| parse_line(line)).enumerate() {
        if idx >= 3 {
            break;
        }
        let sub_res = f(&blueprint, PART2_TIME);
        // println!("{} {}", idx + 1, sub_res);
        res *= sub_res;
    }
    println!("{}", res);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
