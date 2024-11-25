use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
fn rhalf(matrix: &[Vec<i32>], reverse: bool) -> i32 {
    let m = matrix.len();
    let n = if m > 0 { matrix[0].len() } else { return 0 };

    let mut arrsum = vec![vec![0; n]; m];

    let row_range = || {
        if reverse {
            (0..m).rev().collect::<Vec<_>>()
        } else {
            (0..m).collect::<Vec<_>>()
        }
    };

    for i in row_range() {
        for j in 0..n {
            arrsum[i][j] = matrix[i][j]
                + if reverse && i < m - 1 {
                arrsum[i + 1][j]
            } else if !reverse && i > 0 {
                arrsum[i - 1][j]
            } else {
                0
            };
        }
    }

    let mut maximum = i32::MIN;
    for i in row_range() {
        let mut sum = -1;
        for j in 0..n {
            sum = if sum < 0 { arrsum[i][j] } else { sum + arrsum[i][j] };
            maximum = maximum.max(sum);
        }
    }

    maximum
}


fn approx12(matrix: &[Vec<i32>]) -> i32 {
    let m = matrix.len();
    if m == 1 {
        return rhalf(matrix, false);
    }
    let center_row = m / 2;

    let top = rhalf(&matrix[..center_row], true);
    let bottom = approx12(&matrix[center_row..]);
    let entire = rhalf(matrix, false);

    *[top, bottom, entire].iter().max().unwrap()
}

fn generate_random_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(-20..20)).collect())
        .collect()
}

fn benchmark(c: &mut Criterion) {
    let matrix = generate_random_matrix(8, 4); // 8x4 크기의 랜덤 행렬 생성
    c.bench_function("matrix calculation", |b| {
        b.iter(|| {
            approx12(&matrix)
        })
    });
}


criterion_group!(benches, benchmark);
criterion_main!(benches);

// fn main() {
//     let mat = vec![
//         vec![1, 2, -1, -4, -20],
//         vec![-8, -3, 4, 2, 1],
//         vec![3, 8, 10, 1, 3],
//         vec![-4, -1, 1, 7, -6],
//     ];
//
//
//     println!(
//         "1/2배 이상임이 보장되는 최대 부분행렬합의 근사해: {}",
//         approx12(&mat)
//     );
// }
