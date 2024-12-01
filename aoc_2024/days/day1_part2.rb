module Aoc
  class Day < DayBase
    def run
      col1 = icolumns[0].sort
      col2 = icolumns[1].sort

      col1.inject(0) do |score, col1_entry|
        appearances = col2.select { |col2_entry| col2_entry == col1_entry }.size
        score += (col1_entry * appearances)
      end
    end
  end
end
