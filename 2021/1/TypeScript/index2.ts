import fs from "fs";

const input = fs
  .readFileSync("./input.txt", "utf8")
  .trim()
  .split("\n")
  .map((line: string) => Number(line));

// Test Input
// const input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

let count = 0;

for (let i = 0; i <= input.length - 3; i += 1) {
  const window = [input[i], input[i + 1], input[i + 2]].reduce(
    (a, b) => a + b,
    0
  );
  const nextWindow = [input[i + 1], input[i + 2], input[i + 3]].reduce(
    (a, b) => a + b,
    0
  );

  if (nextWindow > window) {
    count += 1;
  }
}

console.log(count);
