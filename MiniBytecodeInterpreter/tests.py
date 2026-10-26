# ।। ॐ नमः शिवाय ।। 
# @date 25th October, 2026

import time

def run_addition_test(max_val):
    start_time = time.perf_counter()
    total_sum = 0

    for i in range(1, max_val + 1):
        total_sum += i

    time_ns = (time.perf_counter() - start_time) * 1e9
    # print(f"Time for Python to add up to {max_val}: {time_ns:.0f}ns.")
    # print(f"Sum up to {max_val} by Python: {total_sum}.")

    return total_sum, time_ns

run_addition_test(100)
run_addition_test(150)
run_addition_test(200)

def run_heavy_addition_test(max_val, times):
    total_time = 0
    cached_sum = 0
    cached_time_ns = 0

    for _ in range(times):
        cached_sum, cached_time_ns = run_addition_test(max_val)
        total_time += cached_time_ns

    avg_time = total_time / times

    print(f"Average time for Python to add up to {max_val} across {times} runs: {avg_time:.0f}ns.")

    return avg_time

run_heavy_addition_test(100, 1000) # Average time for Python to add up to 100 across 1000 runs: 5140ns.
