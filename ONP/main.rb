#!/usr/bin/env ruby

class Stack
    def initialize(size)
        @stack = Array.new(size)
        @pos = -1
    end

    def push(element)
        if @pos < @stack.size.pred #Czy mogę dodać 1 do pozycji i nadal będę jeszcze się mieścił w indeksach
            @pos = @pos + 1
            @stack[@pos] = element 
        end
    end

    def top
        return @pos >= 0 ? @stack[@pos] : nil
    end

    def pop
        if @pos >= 0
            @stack[@pos]
            @pos = @pos - 1
        end
    end

    def empty?
        return @pos < 0 
    end

    def clear
        @pos = -1
    end
end

LEFT = true
RIGHT = false

s = Stack.new 450
priority = { nil => 0, '+' => 1, '-' => 1, '*' => 2, '/' => 2, '^' => 3, '(' => 4, ')' => 4 }

gets.to_i.times do 
    s.clear
    exp = gets
    exp.chomp!
    exp.chomp!

    exp.each_char do |c|
        if c.ord >= 'a'.ord && c.ord <= 'z'.ord
            print c
        elsif s.empty? || s.top == '('
            s.push c
        elsif c == ')'
            while s.top != '(' do
                print s.top
                s.pop
            end
            
            s.pop
        elsif priority[s.top] < priority[c]
            s.push c
        else
            while !s.empty? && priority[s.top] >= priority[c] do
                print s.top
                s.pop
            end

            s.push c
        end
    end

    until s.empty?
        print s.top
        s.pop
    end
    putc "\n".ord
end