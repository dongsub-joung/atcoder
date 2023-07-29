import sys

input= sys.stdin.readline

N= int(input())
mochi= []

while 1:
    if N == 0:
        break
    mochi.append(int(input()))
    N-=1

seted= set(mochi)
print(len(seted))
