use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use rand::Rng;
use rand::seq::SliceRandom; 

const SQUARE_SIZE: i32 = 10; // Size of each square in the grid
const MAX_LINE_LENGTH: usize = 10; // Maximum length of each line

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let svg = generate_svg(&grid);
    svg::save("output.svg", &svg).unwrap();
}

fn generate_svg(grid: &Vec<Vec<i32>>) -> Document {
    let width = grid[0].len() as i32 * SQUARE_SIZE;
    let height = grid.len() as i32 * SQUARE_SIZE;
    let mut document = Document::new().set("width", width).set("height", height);

    let mut rng = rand::thread_rng();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    while let Some((start_x, start_y)) = find_random_start(&grid, &mut visited, &mut rng) {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut data = Data::new().move_to((current_x * SQUARE_SIZE as usize, current_y * SQUARE_SIZE as usize));

        for _ in 0..MAX_LINE_LENGTH {
            if let Some((next_x, next_y)) = find_next_step(&grid, current_x, current_y, &mut visited, &mut rng) {
                current_x = next_x;
                current_y = next_y;
                data = data.line_to((current_x * SQUARE_SIZE as usize, current_y * SQUARE_SIZE as usize));
            } else {
                break;
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);

        document = document.add(path);
    }

    document
}

fn find_random_start(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, rng: &mut impl Rng) -> Option<(usize, usize)> {
    // Find available starts without modifying visited.
    let available_starts = find_available_starts(grid, visited);

    if available_starts.is_empty() {
        None
    } else {
        let index = rng.gen_range(0..available_starts.len());
        let (x, y) = available_starts[index];
        visited[y][x] = true;
        Some((x, y))
    }
}

fn find_available_starts(grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &cell)| {
                    if cell == 1 && !visited[y][x] {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .collect()
}



fn find_next_step(grid: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, rng: &mut impl Rng) -> Option<(usize, usize)> {
    let mut possible_steps = vec![];

    for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_x < grid[0].len() as i32 && new_y >= 0 && new_y < grid.len() as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if grid[new_y][new_x] == 1 && !visited[new_y][new_x] {
                possible_steps.push((new_x, new_y));
            }
        }
    }

    if !possible_steps.is_empty() {
        let index = rng.gen_range(0..possible_steps.len());
        let (next_x, next_y) = possible_steps[index];
        visited[next_y][next_x] = true;
        Some((next_x, next_y))
    } else {
        None
    }
}
