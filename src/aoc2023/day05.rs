use aochelpers;

type Transformation = (std::ops::Range<usize>, i64);

pub fn run() {
    let input = aochelpers::get_daily_input(5, 2023).unwrap();
    println!("Part One Solution: {}", solve_part_one(input));
    // println!("Part Two Solution: {}", solve_part_two("src/input.txt".to_string()));
}

fn solve_part_one(input: String) -> i64 {
    let blocks: Vec<&str> = input.split("\n\n").collect();

    // Parse seeds
    let seeds: Vec<i64> = blocks[0][6..]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Parse map layers
    let map_layers: Vec<Vec<Transformation>> = blocks[1..]
        .iter()
        .map(|&b| parse_map(b))
        .collect();

    // Process seeds with transformations
    let result: Vec<i64> = seeds
        .iter()
        .map(|&seed| {
            map_layers.iter().fold(seed, |current_seed, transformations| {
                mask_number(current_seed, transformations)
            })
        })
        .collect();

    // Return the minimum result
    *result.iter().min().unwrap_or(&0)
}

// fn solve_part_two(filename: String) -> usize {
//     let input = fs::read_to_string(filename).unwrap();
//     let blocks: Vec<&str> = input.split("\n\n").collect();

//     // Parse seeds
//     let seeds: Vec<std::ops::Range<usize>> = blocks[0][6..]
//         .split_whitespace()
//         .map(|s| s.parse::<usize>().unwrap())
//         .tuples()
//         .map(|(start, size)| start..start + size)
//         .collect();

//     // Parse transformations
//     let transformations: Vec<Vec<Transformation>> = blocks[1..]
//         .iter()
//         .map(|&b| parse_map(b))
//         .collect();

//     // Apply transformations
//     let mut result: Vec<usize> = Vec::new();
//     for seed_range in &seeds {
//         let mut ranges: Vec<std::ops::Range<usize>> = vec![seed_range.clone()];

//         for block in &transformations {
//             ranges = ranges
//                 .into_iter()
//                 .flat_map(|r| apply_transformations(r, block.clone()))
//                 .collect();
//         }

//         result.push(ranges.into_iter().min_by_key(|r| r.start).unwrap().start);
//     }

//     *result.iter().min().unwrap_or(&0)
// }

fn parse_map(map_block: &str) -> Vec<Transformation> {
    let transformations: Vec<Transformation> = map_block
        .lines()
        .skip(1) // Skip the first line
        .map(|line| parse_range(line))
        .collect();

    let mut sorted_transformations = transformations;
    sorted_transformations.sort_by(|a, b| a.0.start.cmp(&b.0.start));

    sorted_transformations
}

fn mask_number(num: i64, transformations: &[Transformation]) -> i64 {
    for (mask, offset) in transformations {
        if mask.contains(&(num as usize)) {
            return num + offset;
        }
    }
    num
}

fn parse_range(line: &str) -> Transformation {
    let mut parts = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let dest_start = parts.next().unwrap();
    let source_start = parts.next().unwrap();
    let size = parts.next().unwrap();

    (source_start..source_start + size, (dest_start as i64) - (source_start as i64))
}

fn do_ranges_overlap(a: std::ops::Range<usize>, b: std::ops::Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn shift_range(r: std::ops::Range<usize>, offset: usize) -> std::ops::Range<usize> {
    (r.start + offset)..(r.end + offset)
}

// fn apply_transformations(base: std::ops::Range<usize>, transforms: &[Transformation]) -> Vec<std::ops::Range<usize>> {
//     for (mask, offset) in transforms {
//         // 1 - no overlap; skip this mask
//         if !do_ranges_overlap(base.clone(), mask.clone()) {
//             continue;
//         }

//         // 2 - base is inside mask, shift entire base
//         if mask.start <= base.start && base.end <= mask.end {
//             return vec![shift_range(base, *offset)];
//         }

//         // 3 - mask is a subset of base
//         // return unshifted left, shifted middle, and recurse for the rest
//         if base.start <= mask.start && mask.end <= base.end {
//             return vec![
//                 base.start..mask.start,
//                 shift_range(mask.clone(), *offset),
//                 apply_transformations(mask.end..base.end, transforms).into_iter().flatten(),
//             ].collect();
//         }

//         // 4 - mask overlaps only the left side,
//         // return masked left, recurse for the rest
//         if mask.start <= base.start && mask.end <= base.end {
//             return vec![
//                 shift_range(base.start..mask.end, *offset),
//                 apply_transformations(mask.end..base.end, transforms).into_iter().flatten(),
//             ].collect();
//         }

//         // 5 - mask overlaps only the right side
//         // return unshifted left, masked right
//         if base.start <= mask.start && base.end <= mask.end {
//             return vec![
//                 base.start..mask.start,
//                 shift_range(mask.start..base.end, *offset),
//             ];
//         }
//     }

//     // no masks overlapped this base; pass it through
//     vec![base]
// }

#[cfg(test)]
mod day_5 {
    use itertools::Itertools;

    #[test]
    fn part_one() {
        let mut lines = include_str!("input.txt").lines();
        let mut seeds = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|n| (n.parse::<usize>().unwrap(), false))
            .collect_vec();

        for line in lines {
            if !line.is_empty() && !line.contains("map") {
                let records = line
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().expect("failed to parse record number"))
                    .collect::<Vec<_>>();

                let destination_range_start = records[0];
                let source_range_start = records[1];
                let range_length = records[2];

                seeds = seeds
                    .iter()
                    .map(|(seed, seen)| {
                        if !seen
                            && *seed >= source_range_start
                            && *seed <= (source_range_start + range_length)
                        {
                            (destination_range_start + (*seed - source_range_start), true)
                        } else {
                            (*seed, *seen)
                        }
                    })
                    .collect();
            } else {
                seeds = seeds.iter().map(|(seed, _)| (*seed, false)).collect()
            }
        }

        let result = seeds.iter().map(|(seed, _)| seed).min().unwrap();

        assert_eq!(*result, 240320250);
    }

    #[test]
    fn part_two() {
        let mut lines = include_str!("input.txt").lines();

        let seed_ranges = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec()
            .chunks(2)
            .map(|c| c[0]..(c[0] + c[1]))
            .collect_vec();

        let mut maps: Vec<Vec<(usize, usize, usize)>> = vec![];
        let mut temp_map: Option<Vec<(usize, usize, usize)>> = None;

        lines.skip(1).for_each(|line| {
            match line.trim() {
                line if line.ends_with("map:") => {
                    // NEW MAP
                    temp_map = Some(vec![]);
                }
                line if line.is_empty() => {
                    // END OF MAP
                    maps.push(temp_map.as_mut().expect("No current map!").clone());
                    temp_map = None;
                }
                line => {
                    // VALUE IN CURRENT MAP
                    let records = line
                        .trim()
                        .split(' ')
                        .map(|x| x.parse::<usize>().expect("failed to parse record number"))
                        .collect_vec();

                    temp_map
                        .as_mut()
                        .expect("No current map!")
                        .push((records[0], records[1], records[2]));
                }
            };
        });

        if let Some(current_map) = temp_map {
            // catch hanging map at end of file
            maps.push(current_map);
        }

        let in_range = |seed: usize| seed_ranges.iter().any(|x| x.start <= seed && x.end >= seed);

        let get_seed_location = |mut step: usize| -> usize {
            for map in maps.iter().rev() {
                for (destination_range_start, source_range_start, range_length) in map {
                    if destination_range_start <= &step
                        && destination_range_start + range_length > step
                    {
                        step = source_range_start + step - destination_range_start;
                        break;
                    }
                }
            }

            step
        };

        let mut answer: Option<usize> = None;
        let mut index: usize = 0;
        while answer.is_none() {
            let location = get_seed_location(index);
            if in_range(location) {
                answer = Some(index);
            }

            index += 1;
        }

        assert_eq!(answer.unwrap(), 78775051);
    }
}
