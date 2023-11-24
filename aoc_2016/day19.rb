#!/bin/ruby

class WhiteElephant
  def initialize(player_count)
    @player_count = player_count
  end

  def play!
    elves = (0..(@player_count - 1)).to_a
    while elves.size > 1 do
      elves.delete_at(1)
      elves = elves.rotate
    end
    elves.first + 1
  end

  def guess!
    mod = modulos.select { |m| m <= @player_count }.last
    (2 * (@player_count % mod)) + 1
  end

  private

  def modulos
    arr = [1]
    while arr.last <= @player_count
      arr.push(arr.last * 2)
    end
    arr
  end
end

if true
  (1..50).to_a.each do |x|
    game = WhiteElephant.new(x)
    actual_winner = game.play!
    guessed_winner = game.guess!
    raise "WRONG #{x}, #{actual_winner}, #{guessed_winner}" unless actual_winner == guessed_winner
    puts "#{actual_winner} wins with #{x} participants. guessed #{guessed_winner}"
  end
else
  input = 3018458
  game = WhiteElephant.new(input)
  puts "#{game.guess!} wins ... (we guess :shrug:)"
end
