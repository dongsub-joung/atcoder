import sys

input= sys.stdin.readline

n= int(input())
s= input()

char_array= [c for c in s]
flag= [False] *3
cnt=0

while True:
    if flag[0] and flag[1] and flag[2]:
        break

    buff= char_array.pop(0)
    cnt+=1
    if buff == 'A':
        flag[0]= True
    elif buff == 'B':
        flag[1]= True
    elif buff == 'C':
        flag[2]= True

print(cnt)