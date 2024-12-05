module Aoc
  class Day < DayBase
    attr_reader :rules, :updates

    def run
      sections = raw.split(/\n\n/)
      @rules = to_rows(sections[0]).map { |row| row.first.split("|").map(&:to_i) }
      @updates = to_rows(sections[1]).map { |row| row.first.split(",").map(&:to_i) }
      ordered_updates = invalid_updates.map { |update| ordered_update(update) }
      middle_numbers(ordered_updates).sum
    end

    private

    def middle_numbers(updates)
      updates.map do |update|
        index = (update.size / 2).to_i
        update[index]
      end
    end

    def ordered_update(update)
      rules.each do |rule|
        update.each do |num|
          rule1_index = update.index(rule[0])
          rule2_index = update.index(rule[1])

          if [rule1_index, rule2_index].all? && rule1_index > rule2_index
            rule2_val = update[rule2_index]
            rule1_val = update[rule1_index]
            @ordered_update = update
            @ordered_update[rule1_index] = rule2_val
            @ordered_update[rule2_index] = rule1_val
            ordered_update(@ordered_update)
          end
        end
      end

      update
    end

    def invalid_updates
      updates.reject do |update|
        rules.all? do |rule|
          next(true) if update.index(rule[0]).nil? || update.index(rule[1]).nil?
          update.index(rule[0]) < update.index(rule[1])
        end
      end
    end
  end
end
