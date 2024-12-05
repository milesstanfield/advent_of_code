require "pry"

module Aoc
  class DayBase
    attr_reader :rows, :raw

    def initialize(raw)
      @raw = raw # "1   2\n3   4\n5   6"
      @rows = raw.split(/\n/).map(&:split) # [["1", "2"], ["3", "4"], ["5", "6"]]
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

  def self.run(filename)
    filename ||= Dir["#{File.dirname(__FILE__)}/days/*.rb"].last.split("/").last
    require_relative "days/#{filename}"
    raw = File.read(File.dirname(__FILE__) + "/data")
    puts Day.new(raw).run
  end
end
