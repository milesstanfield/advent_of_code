module Aoc
  class Day < DayBase
    def run
      sections = raw.split(/\n\n/)
      rules = to_rows(sections[0]).map { |row| row.first.split("|").map(&:to_i) }
      updates = to_rows(sections[1]).map { |row| row.first.split(",").map(&:to_i) }

      valid_updates = updates.select do |update|
        rules.all? do |rule|
          next(true) if update.index(rule[0]).nil? || update.index(rule[1]).nil?
          update.index(rule[0]) < update.index(rule[1])
        end
      end

      return 0 if valid_updates.empty?

      valid_updates.map do |update|
        index = (update.size / 2).to_i
        update[index]
      end.sum
    end
  end
end
