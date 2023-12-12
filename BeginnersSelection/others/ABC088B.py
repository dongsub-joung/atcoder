import sys

input= sys.stdin.readline

n= int(input())
As= list(map(int, input().split()))

As.sort()
answer= 0
idx= 0
while 1:
    if idx % 2 == 0:
        answer+= As.pop()
    else:
        answer-= As.pop()
    idx+=1
    n-= 1
    if n == 0:
        break

print(answer)