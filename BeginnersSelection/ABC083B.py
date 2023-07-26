import sys

input= sys.stdin.readline
N, A, B= map(int, input().split())
ans= 0

for i in range(N):
    num= str(i+1)
    size= len(num)
    sumed= 0
    
    for j in range(size):
        sumed+= int(num[j])
    if A <= sumed <= B:
        ans+= int(num)

print(ans)