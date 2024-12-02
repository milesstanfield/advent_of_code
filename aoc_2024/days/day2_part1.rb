module Aoc
  class Day < DayBase
    def run
      irows.map do |row|
        next(false) unless (row == row.sort) || (row == row.sort.reverse)
        safe_row?(row.sort)
      end.select do |safe|
        safe
      end.size
    end

    private

    def safe_row?(row)
      row.each_with_index.map do |level, i|
        next(true) if i == (row.size - 1)
        (1..3).cover?(row[i + 1] - level)
      end.all?
    end
  end
end
