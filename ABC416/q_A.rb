def check(start_dix, end_idx, array)
  i= start_dix
  result= []
  while end_idx > i
    result << small_check(array[i])
    i+=1
  end

  return result
end

def small_check(e)
  if e == "o"
    return true
  else
    return false
  end
end

def true_checker(e)
  result= true
  if e == false
    result= false
  end
  return result
end

s_str= gets.chomp.split(" ")
s= []
s_str.each do |e|
  s << Integer(e)
end

str= gets.chomp.split("")

result= check(s[1]-1, s[2], str)

flag= true
result.each do |e|
  if true_checker(e) == false
    flag= false
  end
end


if flag == true
  puts "Yes"
else
  puts "No"
end