// ।। ॐ नमः शिवाय ।। \\
// @date 23rd October, 2026

function runAdditionTest(max) {
  const start = performance.now();
  let sum = 0;
  for (let k=1; k<=max; k++) {
    sum += k;
  }
  const time = (performance.now() - start) * 1000000;
  return time;
}

function runHeavyAdditionTest(max, times) {
  let totalTime = 0;
  for (let k=0; k<times; k++) {
    totalTime += runAdditionTest(max);
  }
  const averageTime = totalTime / times;
  console.log(`Average time for JS to sum upto ${max} across ${times} runs: ${averageTime}ns.`);
}

function main() {
  runHeavyAdditionTest(100, 1000);
}
main(); // Average time for JS to sum upto 100 across 1000 runs: 2427.190015092492ns.
