# fail 3 case over
import sys

input= sys.stdin.readline

N= int(input())
P_arr= list(map(int,input().split()))

max, pivot= 0,0

for i, p in enumerate(P_arr):
    if i == 0:
        pivot= p
    if max < p:
        max= p

answer= abs((max+1)-pivot)

print(answer)
