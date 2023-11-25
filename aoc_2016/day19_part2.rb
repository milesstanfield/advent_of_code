#!/bin/ruby

class WhiteElephant
  def initialize(player_count)
    @player_count = player_count
  end

  def play!
    elves = (0..(@player_count - 1)).to_a
    while elves.size > 1 do
      elves.delete_at(steal_from_index(elves))
      elves = elves.rotate
    end
    elves.first + 1
  end

  def guess!
    mod = modulos.select { |m| m <= @player_count }.last # [1, 3, 9, 27, 81, 243]
    if @player_count >= (mod * 2)
      (2 * (@player_count % mod)) + mod
    elsif @player_count == mod
      mod
    else
      @player_count % mod
    end
  end

  private

  def steal_from_index(elves)
    shim = elves.size.even? ? 0 : 1
    (elves.size.to_f / 2.to_f).round - shim
  end

  def modulos
    arr = [1]
    while arr.last <= @player_count
      arr.push(arr.last * 3)
    end
    arr
  end
end

if false
  (1..82).each do |x|
    game = WhiteElephant.new(x)
    actual_winner = game.play!
    guessed_winner = game.guess!
    raise "WRONG #{x}, #{actual_winner}, #{guessed_winner}" unless actual_winner == guessed_winner
    puts "player #{actual_winner} wins with #{x} participants. guessed #{guessed_winner}"
  end
else
  input = 3018458
  game = WhiteElephant.new(input)
  puts "#{game.guess!} wins ... (we guess :shrug:)"
end
