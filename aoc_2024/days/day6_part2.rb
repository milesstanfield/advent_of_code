module Aoc
  class Day < DayBase
    def run
      @rows = char_split_rows
      @positions = []
      @obstruction_positions = []
      process_position!(*first_position)
      puts @obstruction_positions
      @obstruction_positions.size
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
      char = rows[row_i][col_i]
      @positions << {row: row_i, col: col_i, char: char}
      rows[row_i][col_i] = "."

      case char
      when "<"
        if (col_i - 1) >= 0
          if guard?(rows[row_i][col_i - 1])
            rows[row_i][col_i] = "^"
            process_position!(row_i, col_i)
          else
            # if its blank (but the space exists), keep going in this direction
            # until you find a non blank one and check if theres an existing pos
            # for that. if there is then put obstruction

            if existing_pos_this_direction?(row_i: row_i, col: col, char: "^")
              @obstruction_positions << {row: row_i, col: col_i - 1}
            end

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
            if @positions.index({row: row_i + 1, col: col_i, char: "v"})
              @obstruction_positions << {row: row_i, col: col_i + 1}
            end

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
            if @positions.index({row: row_i, col: col_i + 1, char: ">"})
              @obstruction_positions << {row: row_i - 1, col: col_i}
            end

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
            if @positions.index({row: row_i, col: col_i - 1, char: "<"})
              @obstruction_positions << {row: row_i + 1, col: col_i}
            end

            rows[row_i + 1][col_i] = char
            process_position!(row_i + 1, col_i)
          end
        end
      end
    end

    def existing_pos_this_direction?(row_i:, col_i:, char:)
      case char
      when "^"
        rows[row_i - 1][col_i]
      end
      # @positions.index({row: row_i - 1, col: col_i, char: "^"})
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
