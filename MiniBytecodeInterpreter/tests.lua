-- ।। ॐ नमः शिवाय ।। 
-- @date 25th October, 2026

local function run_addition_test(max)
  local start_time = os.clock()
  local sum = 0

  for i = 1, max do
    sum = sum + i
  end

  local time_ns = (os.clock() - start_time) * 1e9
  -- print(string.format("Time for Lua to add up to %d: %.0fns.", max, time_ns))
  -- print(string.format("Sum up to %d by Lua: %d.", max, sum))

  return sum, time_ns
end

-- run 
run_addition_test(100)
run_addition_test(150)
run_addition_test(200)

local function run_heavy_addition_test(max, times)
  -- warmup (JIT-friendly for LuaJIT; Lua 5.4 ignores, but good hygiene)
  for _ = 1, 10 do run_addition_test(max) end

  -- AMORTIZED timing: one clock read around N runs
  local t0 = os.clock()
  local sum = 0
  for _ = 1, times do
    sum = 0
    local s = run_addition_test(max)
    sum = sum + s  -- consume so Lua doesn't optimize it away
  end
  local elapsed = (os.clock() - t0) * 1e9
  local per_run = elapsed / times
  print(string.format("Avg per run (%d runs): %.0f ns  [sum=%d]", times, per_run, sum))
end

run_heavy_addition_test(100, 1000000)  -- many more runs, amortized
