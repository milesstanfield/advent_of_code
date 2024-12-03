module Aoc
  class Day < DayBase
    def run
      rows.inject(0) do |total_rows, row|
        total_rows += row.inject(0) do |total_row, input|
          total_row += input.scan(/mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)/).inject(0) do |total, match|
            numbers = match.scan(/\d+/).map(&:to_i)
            multiplication = numbers[0] * numbers[1]
            total += multiplication
          end
        end
      end
    end
  end
end
