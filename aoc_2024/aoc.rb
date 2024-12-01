require "pry"

module Aoc
  class DayBase
    attr_reader :rows

    def initialize(rows)
      @rows = rows # [["1", "2"], ["3", "4"], ["5", "6"]]
    end

    # [[1, 2], [3, 4], [5, 6]]
    def irows
      @irows ||= rows.map { |row| row.map(&:to_i) }
    end

    # [[1, 3, 5], [2, 4, 6]]
    def icolumns
      @icolumns ||= columns.map { |col| col.map(&:to_i) }
    end

    # [["1", "3", "5"], ["2", "4", "6"]]
    def columns
      @columns ||= begin
        rows.each do |row|
          @arr ||= row.size.times.to_a.map { [] }
          row.each_with_index do |entry, i|
            @arr[i] << entry
          end
        end
        @arr
      end
    end
  end

  def self.run_latest
    last_file_name = Dir["#{File.dirname(__FILE__)}/days/*.rb"].last.split("/").last
    require_relative "days/#{last_file_name}"

    rows = File.readlines(File.dirname(__FILE__) + "/data", chomp: true).map do |line|
      line.split
    end

    puts Day.new(rows).run
  end
end
