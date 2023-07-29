import sys

input= sys.stdin.readline

N, Y= map(int, input().split())
money= [10000, 5000, 1000]

ichiman= (Y // money[0])
if ichiman > 0:
    Y-= Y * ichiman
   
elif (Y // money[1]) > 0:
    global gosen
    gosen= (Y // money[1])
    Y-= Y* gosen 
    
elif (Y // money[2])> 0:
    global sen
    sen= (Y // money[2])
    Y-= Y* sen

print(Y)

if Y <= 0:
    amount= [-1, -1, -1]
else:
    amount= [ichiman, gosen, sen]

amount= map(str, amount)
answer= " ".join(amount)
print(answer)
