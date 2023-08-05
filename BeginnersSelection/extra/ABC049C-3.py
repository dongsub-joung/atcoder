import sys

input= sys.stdin.readline

tails= ["eraser", "erase", "dreamer", "dream"]
S= input().strip()

for string in tails:
    S= S.replace(string, '')
    print(S)

if S: 
    print("NO")
else:
    print("YES")