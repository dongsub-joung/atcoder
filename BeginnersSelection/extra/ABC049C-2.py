import sys

input= sys.stdin.readline

S= input()

tails= ["dreamer", "eraser"]
tails2= ["dream", "erase"]
buff= ""
flag= True

for c in S:
    if buff not in tails:
        if buff not in tails2:
            flag= False
    buff+= c

if flag:
    print("YES")
else:
    print("NO")