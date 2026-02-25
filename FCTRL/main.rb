#!/usr/bin/env ruby

power = Array.new(13) { |i| 5**i }

for k in 1..gets.to_i do 
    z = gets.to_i
    zeros = 0
    
    for i in 1..Math.log(z, 5) do
        zeros += z / power[i]
    end

    puts zeros
end