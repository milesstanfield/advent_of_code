module Aoc
  class Day < DayBase
    def run
      @do = true

      raw.size.times.inject(0) do |total, i|
        if raw[i..(i + 3)] == "do()"
          @do = true
        end

        if raw[i..(i + 6)] == "don't()"
          @do = false
        end

        if @do && (match = raw[i..(i + 11)].scan(/^mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)/).first)
          numbers = match.scan(/\d+/).map(&:to_i)
          multiplication = numbers[0] * numbers[1]
          total += multiplication
        end

        total
      end
    end
  end
end
