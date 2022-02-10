// @ts-ignore - Just complaining about types
import fs from "fs";

const input = fs
  .readFileSync("./input.txt", "utf8")
  .trim()
  .split("\n")
  .map((line: string) => Number(line));

let count = 0;
for (let i = 0; i < input.length; i += 1) {
  const current = input[i];
  const previous = input[i - 1];

  if (current > previous) count += 1;
}

console.log(count);
