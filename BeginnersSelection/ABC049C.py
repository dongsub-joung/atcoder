# fail
import sys

input= sys.stdin.readline

str_list= ["dream", "dreamer", "erase", "eraser"]
answer= ""

S= input()
buff= ""

for c in S:
    buff+= c
    if buff in str_list:
        print("YES")
print("NO")