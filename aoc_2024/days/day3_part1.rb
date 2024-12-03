module Aoc
  class Day < DayBase
    def run
      raw.scan(/mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)/).inject(0) do |total, match|
        numbers = match.scan(/\d+/).map(&:to_i)
        multiplication = numbers[0] * numbers[1]
        total += multiplication
      end
    end
  end
end
