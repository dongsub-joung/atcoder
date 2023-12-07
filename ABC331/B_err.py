def minimum_cost_eggs(N, S, M, L):
    # N個以上の卵を買うために必要な最小の金額を求める関数

    # 6個入り、8個入り、12個入りのパックごとの最小金額を計算
    min_cost = float('inf')

    # i, j, kはそれぞれ6個入り、8個入り、12個入りのパックの個数を表す
    for i in range(N // 6 + 1):
        for j in range(N // 8 + 1):
            for k in range(N // 12 + 1):
                if (6 * i + 8 * j + 12 * k >= N):
                    cost = S * i + M * j + L * k
                    min_cost = min(min_cost, cost)

    return min_cost

# 入力を受け取る
N = int(input())  # 卵の個数
S = int(input())  # 6個入りのパックの価格
M = int(input())  # 8個入りのパックの価格
L = int(input())  # 12個入りのパックの価格

# 最小の金額を求める
result = minimum_cost_eggs(N, S, M, L)
print(result)
