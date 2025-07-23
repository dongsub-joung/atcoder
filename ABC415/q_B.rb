s= gets.chomp.split("")

arr= []
i= 1
s.each do |e|
    if e== "#"
        arr << i
    end
    i+=1
end

i=0
j=1
while i < arr.length
    puts "#{arr[i]},#{arr[j]}"
    i+=2
    j+=2
end
