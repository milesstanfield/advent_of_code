module Aoc
  class Day < DayBase
    def run
      @rows = char_split_rows
      @positions = []
      process_position!(*first_position)
      @positions.uniq.size
    end

    private

    def first_position
      rows.each_with_index do |row, row_i|
        row.size.times do |col_i|
          return [row_i, col_i] if cursor?(rows[row_i][col_i])
        end
      end
    end

    def process_position!(row_i, col_i)
      @positions << {row: row_i, col: col_i}
      char = rows[row_i][col_i]
      rows[row_i][col_i] = "."

      case char
      when "<"
        if (col_i - 1) >= 0
          if guard?(rows[row_i][col_i - 1])
            rows[row_i][col_i] = "^"
            process_position!(row_i, col_i)
          else
            rows[row_i][col_i - 1] = char
            process_position!(row_i, col_i - 1)
          end
        end
      when ">"
        if rows[row_i][col_i + 1]
          if guard?(rows[row_i][col_i + 1])
            rows[row_i][col_i] = "v"
            process_position!(row_i, col_i)
          else
            rows[row_i][col_i + 1] = char
            process_position!(row_i, col_i + 1)
          end
        end
      when "^"
        if (row_i - 1) >= 0
          if guard?(rows[row_i - 1][col_i])
            rows[row_i][col_i] = ">"
            process_position!(row_i, col_i)
          else
            rows[row_i - 1][col_i] = char
            process_position!(row_i - 1, col_i)
          end
        end
      when "v"
        if rows[row_i + 1]
          if guard?(rows[row_i + 1][col_i])
            rows[row_i][col_i] = "<"
            process_position!(row_i, col_i)
          else
            rows[row_i + 1][col_i] = char
            process_position!(row_i + 1, col_i)
          end
        end
      end
    end

    def guard?(char)
      char == "#"
    end

    def cursor?(char)
      %w[< > v ^].include?(char)
    end
  end
end

#   0123456789
# 0 ....#.....
# 1 .........#
# 2 ..........
# 3 ..#.......
# 4 .......#..
# 5 ..........
# 6 .#..^.....
# 7 ........#.
# 8 #.........
# 9 ......#...
