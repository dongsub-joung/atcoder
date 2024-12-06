cnt= 0

num_list= list(
    map(int, input().split())
)

# way-2
map= dict()
for e in num_list:
    if e in map:
        map[e]+= 1
    else:
        map[e]= 1
    # print(f'{e}: {map[e]}')
    if 2 in map.values() or 4 in map.values():
        cnt+=1
print(cnt)

# way-1
# pivot= 0
# done_list= list()
# for i in range(0,4):
#     pivot= num_list.pop()
#     contained= num_list.count(pivot)
#     done_list.append(pivot)
#     if done_list.count(pivot) == 0 and contained == 4 :
#         cnt+= 2
#     elif done_list.count(pivot) == 0 and contained == 2:
#         cnt+= 1
#     else:
#         continue