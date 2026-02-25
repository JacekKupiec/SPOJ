10.times do
	total = gets.to_i
	diff = gets.to_i
	
	# x + (x - a) = t
	# x = (t + a) / 2
	
	klaudia = (total + diff) / 2
	natalia = klaudia - diff
	
	puts klaudia, natalia 
end