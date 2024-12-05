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
          count += 1 if up_left?(row_i, col_i)
          count += 1 if up_right?(row_i, col_i)
          count += 1 if down_right?(row_i, col_i)
          count += 1 if down_left?(row_i, col_i)
          count += 1 if right?(row_i, col_i)
          count += 1 if left?(row_i, col_i)
          count += 1 if down?(row_i, col_i)
          count += 1 if up?(row_i, col_i)
        end
      end

      count
    end

    private

    def right?(row_i, col_i)
      rows[row_i][col_i..(col_i + 3)].join == "XMAS"
    end

    def left?(row_i, col_i)
      rows[row_i][(col_i - 3)..col_i].join == "SAMX"
    end

    def down?(row_i, col_i)
      columns[col_i][row_i..(row_i + 3)].join == "XMAS"
    end

    def up?(row_i, col_i)
      columns[col_i][(row_i - 3)..row_i].join == "SAMX"
    end

    def up_left?(row_i, col_i)
      rows.at_positive_index(row_i - 3)&.at_positive_index(col_i - 3) == "S" &&
        rows.at_positive_index(row_i - 2)&.at_positive_index(col_i - 2) == "A" &&
        rows.at_positive_index(row_i - 1)&.at_positive_index(col_i - 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "X"
    end

    def up_right?(row_i, col_i)
      rows.at_positive_index(row_i - 3)&.at_positive_index(col_i + 3) == "S" &&
        rows.at_positive_index(row_i - 2)&.at_positive_index(col_i + 2) == "A" &&
        rows.at_positive_index(row_i - 1)&.at_positive_index(col_i + 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "X"
    end

    def down_right?(row_i, col_i)
      rows[row_i + 3]&.at_positive_index(col_i + 3) == "S" &&
        rows[row_i + 2]&.at_positive_index(col_i + 2) == "A" &&
        rows[row_i + 1]&.at_positive_index(col_i + 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "X"
    end

    def down_left?(row_i, col_i)
      rows[row_i + 3]&.at_positive_index(col_i - 3) == "S" &&
        rows[row_i + 2]&.at_positive_index(col_i - 2) == "A" &&
        rows[row_i + 1]&.at_positive_index(col_i - 1) == "M" &&
        rows[row_i]&.at_positive_index(col_i) == "X"
    end
  end
end

#   0123456789
# 0 ....XXMAS.
# 1 .SAMXMS...
# 2 ...S..A...
# 3 ..A.A.MS.X
# 4 XMASAMX.MM
# 5 X.....XA.A
# 6 S.S.S.S.SS
# 7 .A.A.A.A.A
# 8 ..M.M.M.MM
# 9 .X.X.XMASX

#   0123456789
# 0 MMMSXXMASM
# 1 MSAMXMSMSA
# 2 AMXSXMAAMM
# 3 MSAMASMSMX
# 4 XMASAMXAMM
# 5 XXAMMXXAMA
# 6 SMSMSASXSS
# 7 SAXAMASAAA
# 8 MAMMMXMMMM
# 9 MXMXAXMASX
