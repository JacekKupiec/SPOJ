#!/usr/bin/env ruby

#bank_accounts = Array.new 100_000

input = $stdin

for a in 0...input.gets.to_i do 
    n = input.gets.to_i
    bank_accounts = Array.new n

    for t in 0...n do 
         bank_accounts[t] = input.gets
         bank_accounts[t].chop!
         bank_accounts[t].chop!
    end

    bank_accounts.sort!
    c = 1

    for i in 1...n
        if bank_accounts[i] != bank_accounts[i - 1]
            puts "#{bank_accounts[i - 1]} #{c}"
            c = 1
        else
            c += 1
        end
    end

    puts "#{bank_accounts[-1]} #{c}"
    putc 10
    input.gets
end