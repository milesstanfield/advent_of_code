module Aoc
  class Day < DayBase
    # attr_reader :pos

    def run
      @rows = char_split_rows

      rows.each_with_index do |row, row_i|
        right?(row) && (puts "found right in row #{row.join}")

        left?(row) && (puts "found left in row #{row.join}")
      end

      nil
    end

    private

    def right?(row)
      row.each_with_index.any? do |_, char_i|
        row[char_i..(char_i + 3)].join == "XMAS"
      end
    end

    def left?(row)
      row.each_with_index.any? do |_, char_i|
        row[(char_i - 3)..char_i].join == "SAMX"
      end
    end

    def nine_oclock?(row)
      # row.each_with_index.any? do |_, char_i|
      #   row[(char_i - 3)..char_i].join == "SAMX"
      # end
    end
  end
end


# ....XXMAS.
# .SAMXMS...
# ...S..A...
# ..A.A.MS.X
# XMASAMX.MM
# X.....XA.A
# S.S.S.S.SS
# .A.A.A.A.A
# ..M.M.M.MM
# .X.X.XMASX
