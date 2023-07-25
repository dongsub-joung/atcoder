import sys

input= sys.stdin.readline

# 1 try
# def progressing():
#     global cnt
#     cnt=0
#     for A in A_list:
#         if A % 2 != 0:
#             return cnt
#         else:
#             global listing
#             listing= []
#             listing.append(A%2)
#             cnt+=1
#     return cnt

def progressing():
    for A in A_list:
        cnt= 0
        while A%2 == 0:
            cnt+=1
            A//=2
        cnt_list.append(cnt)


N= int(input())
A_list= map(int, input().split())

cnt_list= []
progressing()

print(min(cnt_list))