require "pry"

module Aoc
  class DayBase
    attr_reader :rows

    def initialize(rows)
      @rows = rows
    end
  end

  def self.run_latest
    last_file_name = Dir["#{File.dirname(__FILE__)}/days/*.rb"].last.split("/").last
    require_relative "days/#{last_file_name}"

    rows = File.readlines(File.dirname(__FILE__) + "/data", chomp: true).map do |line|
      line.split
    end

    Day.new(rows).run
  end
end
