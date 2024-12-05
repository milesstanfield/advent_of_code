class Array
  def at_positive_index(i)
    self.at(i) unless i < 0
  end
end

module Aoc
  class Day < DayBase
    def run
      @rows = char_split_rows
      count = 0

      rows.each_with_index do |row, row_i|
        row.size.times do |col_i|
          if (up_right?(row_i, col_i) || down_left?(row_i, col_i)) && (up_left?(row_i, col_i) || down_right?(row_i, col_i))
            count += 1
          end
        end
      end

      count
    end

    private

    def down_right?(row_i, col_i)
      rows.at_positive_index(row_i - 1)&.at_positive_index(col_i - 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "A" &&
        rows.at_positive_index(row_i + 1)&.at_positive_index(col_i + 1) == "S"
    end

    def up_left?(row_i, col_i)
      rows.at_positive_index(row_i + 1)&.at_positive_index(col_i + 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "A" &&
        rows.at_positive_index(row_i - 1)&.at_positive_index(col_i - 1) == "S"
    end

    def down_left?(row_i, col_i)
      rows.at_positive_index(row_i - 1)&.at_positive_index(col_i + 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "A" &&
        rows.at_positive_index(row_i + 1)&.at_positive_index(col_i - 1) == "S"
    end

    def up_right?(row_i, col_i)
      rows.at_positive_index(row_i + 1)&.at_positive_index(col_i - 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "A" &&
        rows.at_positive_index(row_i - 1)&.at_positive_index(col_i + 1) == "S"
    end
  end
end
