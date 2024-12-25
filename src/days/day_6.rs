use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashSet;
use crate::days::ChristmasSaver;

struct Position {
    r: i32,
    c: i32
}

fn read_rows_6() -> (Vec<Vec<char>>, Position) {
    let mut graph = vec![];

    let path = "src/data_files/day6.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut start_position = Position{r: 0, c: 0};

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = vec![];
        let chars: Vec<char> = line.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            row.push(ch);
            if ch == '^' {
                start_position.r = graph.len() as i32;
                start_position.c = i as i32;
            }
        }
        graph.push(row);
    }

    (graph, start_position)
}

impl ChristmasSaver {
    pub fn count_visited_cells(&self) -> i32 {
        let (mut graph, position) = read_rows_6();
        let rows = graph.len() as i32;
        let cols = graph[0].len() as i32;

        let directions = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
        ];

        let mut dir_map = std::collections::HashMap::new();
        dir_map.insert('^', 0);
        dir_map.insert('>', 1);
        dir_map.insert('v', 2);
        dir_map.insert('<', 3);

        let mut visited = HashSet::new();

        // Start the recursive simulation
        let direction = graph[position.r as usize][position.c as usize];
        move_guard(
            &mut graph,
            position.r,
            position.c,
            direction,
            rows,
            cols,
            &directions,
            &dir_map,
            &mut visited,
        );

        visited.len() as i32
    }

    pub fn count_cycle_blocks(&self) -> i32 {
        let mut count = 0;
        let (mut graph, position) = read_rows_6();
        let rows = graph.len() as i32;
        let cols = graph[0].len() as i32;
        let old = graph[position.r as usize][position.c as usize];

        let directions = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
        ];

        let mut dir_map = std::collections::HashMap::new();
        dir_map.insert('^', 0);
        dir_map.insert('>', 1);
        dir_map.insert('v', 2);
        dir_map.insert('<', 3);

        let mut visited = HashSet::new();

        // move guard marks each cell we visit as 'X'
        let direction = graph[position.r as usize][position.c as usize];
        move_guard(
            &mut graph,
            position.r,
            position.c,
            direction,
            rows,
            cols,
            &directions,
            &dir_map,
            &mut visited,
        );
        graph[position.r as usize][position.c as usize] = old;

        // now, for each X, change it to obstacle, and look for cycle
        for i in 0..graph.len() {
            for j in 0..graph[0].len() {
                if graph[i][j] == 'X' {
                    graph[i][j] = '#';
                    let mut visited2 = HashSet::new();

                    if has_cycle(
                        &mut graph,
                        position.r,
                        position.c,
                        direction,
                        rows,
                        cols,
                        &directions,
                        &dir_map,
                        &mut visited2,
                    ) {
                        count += 1;
                    }
                    graph[i][j] = 'X'; 
                }
            }
        }
        count
    }
}

fn move_guard(
    graph: &mut Vec<Vec<char>>,
    x: i32,
    y: i32,
    direction: char,
    rows: i32,
    cols: i32,
    directions: &[(i32, i32)],
    dir_map: &std::collections::HashMap<char, usize>,
    visited: &mut HashSet<(i32, i32)>,
) {
    if x < 0 || x >= rows || y < 0 || y >= cols {
        return;
    }
    visited.insert((x, y));
    graph[x as usize][y as usize] = 'X';

    let dir_idx = dir_map[&direction];
    let (dx, dy) = directions[dir_idx];
    let next_x = x + dx;
    let next_y = y + dy;

    if next_x >= 0
        && next_x < rows
        && next_y >= 0
        && next_y < cols
        && graph[next_x as usize][next_y as usize] == '#'
    {
        let new_direction = match direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => unreachable!(),
        };
        move_guard(
            graph, x, y, new_direction, rows, cols, directions, dir_map, visited,
        );
    } else {
        move_guard(
            graph, next_x, next_y, direction, rows, cols, directions, dir_map, visited,
        );
    }
}

fn has_cycle(
    graph: &mut Vec<Vec<char>>,
    x: i32,
    y: i32,
    direction: char,
    rows: i32,
    cols: i32,
    directions: &[(i32, i32)],
    dir_map: &std::collections::HashMap<char, usize>,
    visited: &mut HashSet<(i32, i32, char)>,
) -> bool {
    if x < 0 || x >= rows || y < 0 || y >= cols {
        return false;
    }
    if visited.contains(&(x,y,direction)) {
        return true;
    }
    visited.insert((x, y, direction));

    let dir_idx = dir_map[&direction];
    let (dx, dy) = directions[dir_idx];
    let next_x = x + dx;
    let next_y = y + dy;

    if next_x >= 0
        && next_x < rows
        && next_y >= 0
        && next_y < cols
        && graph[next_x as usize][next_y as usize] == '#'
    {
        let new_direction = match direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => unreachable!(),
        };
        return has_cycle(
            graph, x, y, new_direction, rows, cols, directions, dir_map, visited,
        );
    } else {
        return has_cycle(
            graph, next_x, next_y, direction, rows, cols, directions, dir_map, visited,
        );
    }
}