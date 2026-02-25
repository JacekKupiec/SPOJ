#!/usr/bin/env ruby

require 'scanf'

gets.to_i.times do 
    a, b = scanf("%d %d")
    puts a * b
end