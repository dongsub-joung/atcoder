n= Integer(gets.chomp)
s= gets.chomp

bool_arr= []
for i in 1..n-1
    first= s[0]

    if i % 2 == 0 
        if first == s[i]
            bool_arr.append(true)
        else 
            bool_arr.append(false)
        end
    else i % 2 == 1 
        if first != s[i]
            bool_arr.append(true)
        else 
            bool_arr.append(false)
        end
    end
end

if n == 1
    puts "Yes"
else
    if bool_arr.include?(false)
        puts "No"
    else
        puts "Yes"
    end
end