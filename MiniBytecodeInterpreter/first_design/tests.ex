# ।। ॐ नमः शिवाय ।। 
# @date 25th October, 2026

defmodule AdditionBenchmark do
  def run_addition_test(max) do
    start_time = System.monotonic_time(:nanosecond)
    
    # Equivalent to the addition loop
    sum = Enum.reduce(1..max, 0, fn i, acc -> acc + i end)

    time_ns = System.monotonic_time(:nanosecond) - start_time
    
    # Uncomment below to print individual run details
    # IO.puts("Time for Elixir to add up to #{max}: #{time_ns}ns.")
    # IO.puts("Sum up to #{max} by Elixir: #{sum}.")

    {sum, time_ns}
  end

  def run do
    run_addition_test(100)
    run_addition_test(150)
    run_addition_test(200)
  end

  def run_heavy_addition_test(max, times) do
    {total_time, cached_sum, last_time_ns} = 
      Enum.reduce(1..times, {0, 0, 0}, fn _, {acc_time, _, _} ->
        {sum, time_ns} = run_addition_test(max)
        {acc_time + time_ns, sum, time_ns}
      end)

    avg_time = total_time / times

    IO.puts("Average time for Elixir to add up to #{max} across #{times} runs: #{Float.round(avg_time, 2)}ns.")

    avg_time
  end
end

# Execute the tests
AdditionBenchmark.run()
AdditionBenchmark.run_heavy_addition_test(100, 1000) 
# Takes about 730ns