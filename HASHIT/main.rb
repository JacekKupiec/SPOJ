#!/usr/bin/env ruby
DELETED = -1

class String
    def hash_it
        reval = 19*self.each_byte.with_index.reduce(0) do |memo, obj| 
            memo + obj[0]*obj[1].next 
        end

        return reval % 101
    end
end

input = $stdin
#input = File.open('1.in', 'r')

input.gets.to_i.times do
    t = Array.new 101
    counter = 0

    input.gets.to_i.times do
        line = input.gets
        line.chomp!
        line.chomp!
        command, str = line.split ':'
        str ||= ''

        case command
        when 'ADD'
            first_del = nil

            20.times do |j|
                idx = (str.hash_it + j*j + 23*j) % 101
                
                if t[idx].nil?
                    first_del = idx if first_del.nil?
                    break
                elsif first_del.nil? && t[idx] == DELETED
                    first_del = idx
                elsif t[idx] == str
                    first_del = nil
                    break 
                end
            end
            
            unless first_del.nil?
                t[first_del] = str
                counter += 1
            end
        when 'DEL'
            20.times do |j|
                idx = (str.hash_it + j*j + 23*j) % 101

                break if t[idx].nil?

                if t[idx] == str 
                    t[idx] = DELETED
                    counter -= 1
                    break
                end
            end
        end
    end

    puts counter
    t.each_with_index do |v, k|
        puts "#{k}:#{v}" if !v.nil? && v != DELETED
    end
end