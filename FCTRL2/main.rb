#!/usr/bin/env ruby

require 'scanf'

gets.to_i.times do 
	n = scanf('%d').first
	factorial = 1
	
	n.downto(2) { |f| factorial *= f }
	puts factorial
end