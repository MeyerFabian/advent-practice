// You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

// Each square on the grid is allocated in a spiral pattern
// starting at a location marked 1 and then counting up while spiraling outward.
// For example, the first few squares are allocated like this:

// 17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ...

// While this is very space-efficient (no squares are skipped),
// requested data must be carried back to square 1
// (the location of the only access port for this memory system)
// by programs that can only move up, down, left, or right.
// They always take the shortest path:
// the Manhattan Distance between the location of the data and square 1.

// For example:

//     Data from square 1 is carried 0 steps, since it's at the access port.
//     Data from square 12 is carried 3 steps, such as: down, left, left.
//     Data from square 23 is carried only 2 steps: up twice.
//     Data from square 1024 must be carried 31 steps.

// How many steps are required to carry the data from the square
// identified in your puzzle input all the way to the access port?


// Calculates the maximum-norm distance of the number, which is also the the n-th circle
// starting from the middle (1).
pub fn norm_max_spiral_pattern(s_n: i32) -> i32 {
    // The n-th circle goes from (2n-1)^2+1 until (2n+1)^2.
    // Calculation here is the opposite way.

    let mut dist = ((s_n - 1) as f64).sqrt() as i32;
    if dist == 0 {
        return 0;
    }
    if dist % 2 == 0 {
        dist -= 1;
    }
    return dist / 2 + 1;
}

// Calculates the 1-norm or manhatten distance of the number.
// ex. used here:
// 17 16 15 14 13
// 18          12
// 19          11
// 20          10
// 21 22 23 24 25
//
pub fn norm_1_spiral_pattern(s_n: i32) -> i32 {
    let nth_circle = norm_max_spiral_pattern(s_n);
    if nth_circle == 0 {
        return 0;
    }

    let nth_odd_circle = (2 * nth_circle - 1);

    //Reindex to 0
    // 7  6  5  4  3
    // 8           2
    // 9           1
    // 10          0
    // 11 12 13 14 16
    let index = s_n - nth_odd_circle * nth_odd_circle - 1;

    //Shift indices to the next projection of the first quadrant
    // 10 9  8  7  6
    // 11          5
    // 12          4 <--
    // 13          3
    // 14 15 16 17 18
    let index_shifted = index + nth_circle + 1;
    //Projections onto first quadrant
    // 2  1  0  3  2
    // 3     ^     1
    // 0     | --> 0
    // 1           3
    // 2  3  0  1  2
    let mut index_fq = index_shifted % (2 * nth_circle);
    if index_fq > nth_circle {
        //Mirror Indizes from first octant
        // 2  1  0  1  2
        // 1         / 1
        // 0        /->0
        // 1           1
        // 2  1  0  1  2
        index_fq = (index_fq - 2 * nth_circle).abs();
    }

    return nth_circle + index_fq;
}

// Fill grid with nxn zeroes
fn fill_grid_with_zeroes(grid: &mut Vec<Vec<i32>>, n: usize) {
    for i in 0..n {
        grid.push(std::iter::repeat(0).take(n).collect());
    }
}
// Calc steps that we go in the same direction [1,1,2,2,3,3,4,4,5,5,6,6,7,..]
fn calc_steps(mv_vec: &mut Vec<usize>, n: usize) {
    let mut steps = 0;
    let mut i = 1;
    loop {
        mv_vec.push(i);
        mv_vec.push(i);

        steps += 2 * i;
        i += 1;
        if steps > (n - 2) * (n - 2) {
            break;
        }
    }
}

// Spiral algorithm
fn sum_spiral(grid: &mut Vec<Vec<i32>>, start: (usize, usize), n: usize, input: i32) {
    //bigger filters then 3x3 would be possible
    let filter1D = [-1, 0, 1];

    //At the start midpoint
    let mut x = start.0;
    let mut y = start.1;

    let mut move_vec = Vec::new();
    calc_steps(&mut move_vec, n);


    //cycle is always the same right,up,left,down
    let mut move_cycle = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut move_cycle_iter = move_cycle.iter().cycle();

    for num_steps in move_vec {
        // get the next move direction
        let &(m_y, m_x) = move_cycle_iter.next().unwrap();
        for step in 0..num_steps {
            // move to the next direction, which is now midpoint for filter
            x = (x as i32 + m_x) as usize;
            y = (y as i32 + m_y) as usize;
            let mut sum = 0;
            //filter around new midpoint
            for j in filter1D.iter() {
                for i in filter1D.iter() {
                    let ix = (x as i32 + i) as usize;
                    let iy = (y as i32 + j) as usize;
                    sum += grid[iy][ix];
                }
            }
            // advent asks for number bigger than my indiv. number so we just abort then
            if (sum > input) {
                println!{"value bigger than input: {}",sum};
                return;
            }
            grid[y][x] = sum;
        }
    }
}
fn main() {
    let input: i32 = 368078;
    println!("norm1 {}", norm_1_spiral_pattern(input));

    let mut grid = Vec::new();
    let grid_length = 15;
    let midpoint = grid_length / 2;

    fill_grid_with_zeroes(&mut grid, grid_length);

    grid[midpoint][midpoint] = 1;

    sum_spiral(&mut grid, (midpoint, midpoint), grid_length - 2, input);
    for i in grid.iter() {
        println!("{:?}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn norm_max_edge_test() {
        assert_eq!(0, norm_max_spiral_pattern(1));
    }
    #[test]
    fn norm_max_test() {
        assert_eq!(1, norm_max_spiral_pattern(2));
        assert_eq!(1, norm_max_spiral_pattern(9));
        assert_eq!(2, norm_max_spiral_pattern(10));
        assert_eq!(2, norm_max_spiral_pattern(25));
        assert_eq!(3, norm_max_spiral_pattern(26));
        assert_eq!(3, norm_max_spiral_pattern(49));
    }
    #[test]
    fn norm_1_edge_test() {
        assert_eq!(0, norm_1_spiral_pattern(1));
    }
    #[test]
    fn norm_1_test() {
        assert_eq!(1, norm_1_spiral_pattern(2));
        assert_eq!(2, norm_1_spiral_pattern(3));
        assert_eq!(2, norm_1_spiral_pattern(9));
        assert_eq!(3, norm_1_spiral_pattern(10));
        assert_eq!(4, norm_1_spiral_pattern(25));
        assert_eq!(5, norm_1_spiral_pattern(26));
        assert_eq!(6, norm_1_spiral_pattern(49));
        assert_eq!(4, norm_1_spiral_pattern(21));
        assert_eq!(7, norm_1_spiral_pattern(50));
        assert_eq!(31, norm_1_spiral_pattern(1024));
    }
}
