const PATTERN: [i16; 4] = [0, 1, 0, -1];

fn main() {
    let mut data = "59768092839927758565191298625215106371890118051426250855924764194411528004718709886402903435569627982485301921649240820059827161024631612290005106304724846680415690183371469037418126383450370741078684974598662642956794012825271487329243583117537873565332166744128845006806878717955946534158837370451935919790469815143341599820016469368684893122766857261426799636559525003877090579845725676481276977781270627558901433501565337409716858949203430181103278194428546385063911239478804717744977998841434061688000383456176494210691861957243370245170223862304663932874454624234226361642678259020094801774825694423060700312504286475305674864442250709029812379"
        .chars().map(|c| c.to_digit(10).unwrap() as i16).collect();

    for _ in 0..100 {
        data = solve(data);
    }

    println!(
        "Output: {}",
        data.drain(0..8)
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

fn solve(data: Vec<i16>) -> Vec<i16> {
    let mut output = Vec::with_capacity(data.len());
    for output_index in 1..=data.len() {
        let mut result = 0;

        for (input_index, value) in data.iter().enumerate() {
            let pattern_index = (input_index + 1) / output_index % PATTERN.len();
            let pattern_value = PATTERN.get(pattern_index).unwrap();
            result += value * pattern_value;
        }

        output.push((result % 10).abs());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        assert_eq!(
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            solve(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        );

        assert_eq!(
            vec![3, 4, 0, 4, 0, 4, 3, 8],
            solve(vec![4, 8, 2, 2, 6, 1, 5, 8]),
        );

        assert_eq!(
            vec![0, 3, 4, 1, 5, 5, 1, 8],
            solve(vec![3, 4, 0, 4, 0, 4, 3, 8]),
        );

        assert_eq!(
            vec![0, 1, 0, 2, 9, 4, 9, 8],
            solve(vec![0, 3, 4, 1, 5, 5, 1, 8]),
        );
    }

    #[test]
    fn example_1() {
        let mut data = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5,
            5, 9, 5,
        ];

        for _ in 0..100 {
            data = solve(data);
        }

        assert_eq!(
            vec![2, 4, 1, 7, 6, 1, 7, 6],
            data.drain(0..8).collect::<Vec<i16>>(),
        )
    }

    #[test]
    fn example_2() {
        let mut data = vec![
            1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8, 9,
            9, 1, 7,
        ];

        for _ in 0..100 {
            data = solve(data);
        }

        assert_eq!(
            vec![7, 3, 7, 4, 5, 4, 1, 8],
            data.drain(0..8).collect::<Vec<i16>>(),
        )
    }
}
