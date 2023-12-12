import sys

input= sys.stdin.readline

A= int(input())
B= int(input())
C= int(input())
X= int(input())
cnt= 0
coins= [500, 100, 50]

for i in range(A+1):
    for j in range(B+1):
        for k in range(C+1):
            if i *coins[0] + j *coins[1] + k *coins[2] == X:
                cnt+=1
print(cnt)
