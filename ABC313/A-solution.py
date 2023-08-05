# https://www.youtube.com/watch?v=VluAlzOH83I

N= int(input())
A= list(map(int, input().split()))

if N == 1:
    print(0)
    exit(0)
    
mx= max(A[1:])
ans= max(0, mx+1-A[0])

print(ans)