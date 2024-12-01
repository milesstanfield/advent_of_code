module Aoc
  class Day < DayBase
    def run
      col1 = rows.map { |row| row[0] }.flatten.map(&:to_i).sort
      col2 = rows.map { |row| row[1] }.flatten.map(&:to_i).sort

      total = 0
      col1.each_with_index do |col1_entry, i|
        total += (col1[i] - col2[i]).abs
      end

      puts total
    end
  end
end
