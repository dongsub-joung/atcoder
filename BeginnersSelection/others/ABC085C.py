import sys

input= sys.stdin.readline

N, Y= map(int, input().split())
ichiman, gosen, sen= -1, -1, -1

for a in range(N+1):
    for b in range(N+1):
        c= N-a-b
        total= 10000*a + 5000*b + 1000*c
        if total == Y:
            ichiman= a
            gosen= b
            sen= c

answer= [ichiman, gosen, sen]
answer= map(str, answer)
print(" ".join(answer))
