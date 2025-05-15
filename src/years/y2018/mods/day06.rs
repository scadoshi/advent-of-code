use std::collections::HashMap;

fn ppoints() -> Vec<(usize, usize)> {
    include_str!("..\\inputs\\day06.txt")
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect("Failed to split_once by a comma: ','");
            (
                x.trim()
                    .parse()
                    .expect(format!("Failed to parse {} to usize", x).as_str()),
                y.trim()
                    .parse()
                    .expect(format!("Failed to parse {} to usize", y).as_str()),
            )
        })
        .collect()
}

fn distance(point1: &(usize, usize), point2: &(usize, usize)) -> usize {
    ((point1.0 as isize - point2.0 as isize).abs()
        + (point1.1 as isize - point2.1 as isize).abs()) as usize
}

#[allow(dead_code)]
pub fn part_one() {
    let ppoints = ppoints();

    println!("{:?}", ppoints);
    let (min_col, max_col) = (
        ppoints
            .iter()
            .min_by_key(|(col, _)| col)
            .expect("max col not found")
            .0,
        ppoints
            .iter()
            .max_by_key(|(col, _)| col)
            .expect("min col not found")
            .0,
    );
    let (min_row, max_row) = (
        ppoints
            .iter()
            .min_by_key(|(_, row)| row)
            .expect("max y not found")
            .1,
        ppoints
            .iter()
            .max_by_key(|(_, row)| row)
            .expect("min y not found")
            .1,
    );

    let mut std_grid: Vec<(usize, usize)> = Vec::new();
    for col in min_col..=max_col {
        for row in min_row..=max_row {
            std_grid.push((col, row));
        }
    }

    let mut gpoints_map: HashMap<(usize, usize), HashMap<(usize, usize), usize>> =
        HashMap::new();
    // map distance from every grid_point to every place_point
    for gpoint in std_grid.iter() {
        // if grid_point is a place_point we can ignore
        if ppoints.contains(gpoint) {
            continue;
        }

        // the actual mapping process
        for ppoint in ppoints.iter() {
            *gpoints_map
                .entry(*gpoint)
                .or_default()
                .entry(*ppoint)
                .or_default() = distance(gpoint, ppoint);
        }
    }

    let adj_gpoints_map: HashMap<(usize, usize), Option<(usize, usize)>> = gpoints_map
        .into_iter()
        .map(|(gpoint, map)| {
            let min_dist = map.iter().map(|(_, dist)| dist).min().expect("min_dist not found");
            let nearest_ppoint: Option<(usize, usize)> = 
                if map.iter().filter(|(_, dist)| *dist == min_dist).count() > 1 {
                None
            } else {
                Some(*map.iter().find(|(_, dist)| *dist == min_dist).expect("ppoint of min_dist not found").0)
            };
            (gpoint, nearest_ppoint)
        })
        .collect();

    


    
    
    
    let d = 1000;
    let mut big_bounds_grid: Vec<(usize, usize)> = Vec::new();
    for col in min_col - d..=max_col + d {
        for row in min_row - d..=max_row + d {
            big_bounds_grid.push((col, row));
        }
    }
}

#[allow(dead_code)]
pub fn part_two() {}
