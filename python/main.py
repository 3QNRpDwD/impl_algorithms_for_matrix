import numpy as np
import timeit

def Rhalf(matrix, reverse=False):
    M, N = matrix.shape 
    arrsum = [[0] * N for _ in range(M)]
    
    row_range = range(M-1, -1, -1) if reverse else range(M) 
    
    for i in row_range:
        for j in range(N):
            if reverse:
                arrsum[i][j] = (
                    matrix[i][j] + (arrsum[i+1][j] if i < M-1 else 0)
                )
            else:
                arrsum[i][j] = (
                    matrix[i][j] + (arrsum[i-1][j] if i > 0 else 0)
                )
    
    maximum = -1e10
    for i in row_range:
        sum = -1
        for j in range(N):
            if sum < 0:
                sum = arrsum[i][j]
            else:
                sum += arrsum[i][j]
            if sum > maximum:
                maximum = sum
    
    return maximum

def approx12(matrix):
    M = matrix.shape[0]
    if M == 1:
        Max = Rhalf(matrix)
        return Max
    center_row = M // 2
    
    top = Rhalf(matrix[:center_row], reverse=True)  # Top 부분은 아래쪽부터 계산
    bottom = approx12(matrix[center_row:])
    entire = Rhalf(matrix)
    # print("개선후 3개 ", top, bottom, entire)
    return max(top, bottom, entire)

def test_approx12(benchmark):
    matrix = np.random.randint(-5, 5, size=(8, 4))
    result = benchmark(approx12, matrix)
    assert result is not None  # 결과 검증용 (필요 시)




# def A12_approx(String, A):
#     M, N = A.shape
#     if M == 1 or N == 1:
#         return max(0, np.max(np.cumsum(A.flatten())))
#     # 센터
#     center_row = M // 2
#     center_col = N // 2
#     # 중심 
#     max_center_row = max(0, np.max(np.cumsum(A[center_row])))
#     max_center_col = max(0, np.max(np.cumsum(A[:, center_col])))    
#     # 재귀
#     top = A12_approx("top"      ,A[:center_row, :])
#     bot = A12_approx("bot"      ,A[center_row:, :])
#     left = A12_approx("left"    ,A[:, :center_col])
#     right = A12_approx("right"  ,A[:, center_col:])
#     #최대값 선택
#     return max(max_center_row, max_center_col, top, bot, left, right)

# print(mat)

# print(A12_approx("", mat))


# import numpy as np

# def Rhalf(matrix): # 리펙토링 필요 *top dml Rhalf 를 구할때 아래쪽부터 구해야함*
#     M, N = matrix.shape
#     # print("M, N", String, M, N)
#     arrsum = [[0] * N for i in range(M)]
#     for i in range(M):
#         for j in range(N):
#             arrsum[i][j] = matrix[i][j] + (arrsum[i-1][j] if i > 0 else 0)
#             # print("arrsum",String ,arrsum,"matrix", matrix[i][j], "+", (arrsum[i-1][j] if i > 0 else 0))
#     # arrsum = np.cumsum(matrix, axis=0)
#     maximum = -1e10
#     for i in range(M):
#         sum = -1
#         for j in range(N):
#             if sum < 0:
#                 sum = arrsum[i][j]
#             else:
#                 sum += arrsum[i][j]
#             if sum > maximum:
#                 maximum = sum
#         #     print("arrsum2 ",String ,arrsum)
#         # print("maximum",String ,maximum)
            
#     return maximum

# def approx12(matrix):
#     M = matrix.shape[0]
#     # print("M", String, M)
#     if M == 1:
#         Max = Rhalf(matrix)
#         # print("Max", String, M)
#         return Max
#     center_row = M // 2
#     # print("center_row ", String, center_row)
#     # print("matrix ",String , matrix[:center_row])
    
#     top = approx12(matrix[:center_row]) # 리펙토링 필요 *top dml Rhalf 를 구할때 아래쪽부터 구해야함*
#     bottom = approx12(matrix[center_row:])
#     entire = Rhalf(matrix)
#     print("개선전 3개 ", top, bottom, entire)
#     return max(top, bottom, entire)

# mat = np.random.randint(-5, 5, size=(8, 4))

# print("입력된 행렬 A:\n", mat)

# print("개선전: \n1/2배 이상임이 보장되는 최대 부분행렬합의 근사해",approx12(mat))

