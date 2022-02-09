// @ts-ignore - Just complaining about types
import fs from "fs";

const inputFile = fs.readFileSync("./input.txt", "utf8");
const input = inputFile.split("\n").map((line: string) => Number(line));

// 1154
