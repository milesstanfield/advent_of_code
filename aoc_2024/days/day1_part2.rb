module Aoc
  class Day < DayBase
    def run
      col1 = rows.map { |row| row[0] }.flatten.map(&:to_i).sort
      col2 = rows.map { |row| row[1] }.flatten.map(&:to_i).sort

      score = 0
      col1.each_with_index do |col1_entry, i|
        appearances = col2.select { |col2_entry| col2_entry == col1_entry }.size
        score += (col1_entry * appearances)
      end

      puts score
    end
  end
end
