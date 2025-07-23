arr= []

n= gets.chomp
buf= gets.chomp.split
buf.each do |e|
    arr << Integer(e)
end
x= Integer(gets.chomp)

result=  (arr.include? x) ? "Yes" : "No"
puts result