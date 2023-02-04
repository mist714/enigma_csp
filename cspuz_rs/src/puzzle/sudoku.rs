use super::util;
use crate::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use crate::solver::Solver;

pub fn solve_sudoku(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);
    if h != w {
        return None;
    }
    let n = h;
    let mut s = None;
    for i in 2..=5 {
        if n == i * i {
            s = Some(i);
            break;
        }
    }
    let s = s?;

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((n, n), 1, n as i32);
    solver.add_answer_key_int(num);

    for i in 0..n {
        solver.all_different(num.slice_fixed_y((i, ..)));
        solver.all_different(num.slice_fixed_x((.., i)));
    }
    for i in 0..s {
        for j in 0..s {
            solver.all_different(num.slice((((i * s)..((i + 1) * s)), ((j * s)..((j + 1) * s)))));
        }
    }
    for y in 0..n {
        for x in 0..n {
            if let Some(val) = clues[y][x] {
                if val > 0 {
                    solver.add_expr(num.at((y, x)).eq(val));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub fn solve_sudoku_as_cands(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Vec<bool>>>> {
    let (h, w) = util::infer_shape(clues);
    if h != w {
        return None;
    }
    let n = h;
    let mut s = None;
    for i in 2..=5 {
        if n == i * i {
            s = Some(i);
            break;
        }
    }
    let s = s?;

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((n, n), 1, n as i32);
    let mut cands = vec![];
    for _ in 0..n {
        let mut row = vec![];
        for _ in 0..n {
            let b = solver.bool_var_1d(n);
            solver.add_answer_key_bool(&b);
            row.push(b);
        }
        cands.push(row);
    }

    for y in 0..n {
        for x in 0..n {
            for i in 0..n {
                solver.add_expr(num.at((y, x)).eq((i + 1) as i32).iff(cands[y][x].at(i)));
            }
        }
    }

    for i in 0..n {
        solver.all_different(num.slice_fixed_y((i, ..)));
        solver.all_different(num.slice_fixed_x((.., i)));
    }
    for i in 0..s {
        for j in 0..s {
            solver.all_different(num.slice((((i * s)..((i + 1) * s)), ((j * s)..((j + 1) * s)))));
        }
    }
    for y in 0..n {
        for x in 0..n {
            if let Some(val) = clues[y][x] {
                if val > 0 {
                    solver.add_expr(num.at((y, x)).eq(val));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| {
        let mut ret = vec![];
        for y in 0..n {
            let mut row = vec![];
            for x in 0..n {
                row.push(
                    f.get(&cands[y][x])
                        .into_iter()
                        .map(|x| x.unwrap_or(true))
                        .collect(),
                );
            }
            ret.push(row);
        }
        ret
    })
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "sudoku", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["sudoku"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        // generated by cspuz
        vec![
            vec![None, None, None, None, None, Some(8), None, Some(1), None],
            vec![Some(7), None, None, None, Some(2), None, None, None, Some(9)],
            vec![Some(9), None, None, None, None, None, None, None, None],
            vec![None, Some(2), None, Some(3), None, None, Some(7), Some(5), None],
            vec![None, None, None, None, None, None, None, None, None],
            vec![None, Some(1), Some(9), None, None, Some(5), None, Some(4), None],
            vec![None, None, None, None, None, None, None, None, Some(8)],
            vec![Some(3), None, None, None, Some(4), None, None, None, Some(6)],
            vec![None, Some(4), None, Some(5), None, None, None, None, None],
        ]
    }

    #[test]
    fn test_sudoku_problem() {
        let problem = problem_for_tests();
        let ans = solve_sudoku(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        #[rustfmt::skip]
        let expected = vec![
            vec![Some(2), Some(6), Some(4), Some(7), Some(9), Some(8), Some(3), Some(1), Some(5)],
            vec![Some(7), Some(3), Some(5), Some(1), Some(2), Some(4), Some(8), Some(6), Some(9)],
            vec![Some(9), Some(8), Some(1), Some(6), Some(5), Some(3), Some(2), Some(7), Some(4)],
            vec![Some(4), Some(2), Some(6), Some(3), Some(8), Some(9), Some(7), Some(5), Some(1)],
            vec![Some(5), Some(7), Some(3), Some(4), Some(1), Some(6), Some(9), Some(8), Some(2)],
            vec![Some(8), Some(1), Some(9), Some(2), Some(7), Some(5), Some(6), Some(4), Some(3)],
            vec![Some(1), Some(5), Some(2), Some(9), Some(6), Some(7), Some(4), Some(3), Some(8)],
            vec![Some(3), Some(9), Some(7), Some(8), Some(4), Some(1), Some(5), Some(2), Some(6)],
            vec![Some(6), Some(4), Some(8), Some(5), Some(3), Some(2), Some(1), Some(9), Some(7)],
        ];
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_sudoku_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?sudoku/9/9/k8g1g7i2i99o2g3h75q19h5g4o83i4i6g4g5k";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
