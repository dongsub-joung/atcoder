import sys

input= sys.stdin.readline

N= int(input())
P_arr= list(map(int,input().split()))
answer= 0

max_value= max(P_arr)
first= P_arr.pop(0)
if max_value == first:
    print(0)
    exit(1)

max_v= max(P_arr)

answer= max_v-first+1
if answer < 0:
    answer=0
print(answer)
