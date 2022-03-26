use crate::graph;
use crate::solver::Solver;

pub fn solve_slitherlink(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridFrameIrrefutableFacts> {
    let h = clues.len();
    assert!(h > 0);
    let w = clues[0].len();

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridFrame::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    graph::single_cycle_grid_frame(&mut solver, &is_line);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(is_line.cell_neighbors((y, x)).count_true().eq(n));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_slitherlink_problem() {
        // original example: http://pzv.jp/p.html?slither/4/4/dgdh2c7b
        let problem = [
            vec![Some(3), None, None, None],
            vec![Some(3), None, None, None],
            vec![None, Some(2), Some(2), None],
            vec![None, Some(2), None, Some(1)],
        ];
        let ans = solve_slitherlink(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = graph::BoolGridFrameIrrefutableFacts {
            horizontal: vec![
                vec![Some(true), Some(true), Some(true), Some(true)],
                vec![Some(true), Some(false), Some(true), Some(false)],
                vec![Some(true), Some(false), Some(false), Some(false)],
                vec![Some(false), Some(true), Some(false), Some(true)],
                vec![Some(true), Some(false), Some(false), Some(false)],
            ],
            vertical: vec![
                vec![Some(true), Some(false), Some(false), Some(false), Some(true)],
                vec![Some(false), Some(true), Some(true), Some(true), Some(true)],
                vec![Some(true), Some(false), Some(true), Some(true), Some(true)],
                vec![Some(true), Some(true), Some(false), Some(false), Some(false)],
            ],
        };
        assert_eq!(ans, expected);
    }
}