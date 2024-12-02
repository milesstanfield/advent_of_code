module Aoc
  class Day < DayBase
    def run
      irows.map do |row|
        next(true) if safe_row?(row)
        row.each_with_index.map do |r, i|
          mod_row = row.dup
          mod_row.delete_at(i)
          safe_row?(mod_row)
        end.any?
      end.select { |safe| safe }.size
    end

    private

    def safe_row?(row)
      return false unless (row == row.sort) || (row == row.sort.reverse)
      sorted_row = row.sort
      sorted_row.each_with_index.map do |level, i|
        next(true) if i == (row.size - 1)
        (1..3).cover?(sorted_row[i + 1] - level)
      end.all?
    end
  end
end
