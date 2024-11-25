fn rhalf(matrix: &[Vec<i32>], reverse: bool) -> i32 {
    let m = matrix.len();
    let n = if m > 0 { matrix[0].len() } else { return 0 };
                                                        // M, N = matrix.shape  파이썬 코드

    let mut arrsum = vec![vec![0; n]; m]; // 파이썬 코드 : arrsum = [[0] * N for _ in range(M)]

    let row_range = || {
        if reverse { (0..m).rev().collect::<Vec<_>>() }
        else { (0..m).collect::<Vec<_>>() }
    };                                                 // 파이썬 코드: row_range = range(M-1, -1, -1) if reverse else range(M)

    for i in row_range() {                      // 파이썬 코드:  for i in row_range:
        for j in 0..n {                      // 파이썬 코드:     for j in range(N):
            arrsum[i][j] = matrix[i][j]
                + if reverse && i < m - 1 { arrsum[i + 1][j] }
            else if !reverse && i > 0 { arrsum[i - 1][j] } else { 0 };
        }
    }
        /* 파이썬 코드 :
            if reverse:
                arrsum[i][j] = matrix[i][j] + (arrsum[i+1][j] if i < M-1 else 0)
            else:
                arrsum[i][j] = matrix[i][j] + (arrsum[i-1][j] if i > 0 else 0)
        */

    let mut maximum = i32::MIN;   // 파이썬 코드: maximum = -1e10
    for i in row_range() {      // 파이썬 코드: for i in row_range:
        let mut sum = -1;        // 파이썬 코드: sum = -1
        for j in 0..n {     // 파이썬 코드: for j in range(N):
            sum =
                if sum < 0 { arrsum[i][j] }
                else { sum + arrsum[i][j] };
            maximum = maximum.max(sum);
        }
        /*파이썬 코드:
            if sum < 0:
                sum = arrsum[i][j]
            else:
                sum += arrsum[i][j]
            if sum > maximum:
                maximum = sum
         */
    }

    maximum // 파이썬 코드: return maximum
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

fn main() {
    let mat = vec![
        vec![1, 2, -1, -4, -20],
        vec![-8, -3, 4, 2, 1],
        vec![3, 8, 10, 1, 3],
        vec![-4, -1, 1, 7, -6],
    ];


    println!(
        "1/2배 이상임이 보장되는 최대 부분행렬합의 근사해: {}",
        approx12(&mat)
    );
}
