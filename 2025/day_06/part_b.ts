import * as fd from "node:fs";

const file_name = Deno.args[0];
const file: string[] = fd.readFileSync(file_name, { encoding: "utf-8" })
  .split("\n")
  .filter((row) => row.length > 0);
let max_size = file[0].length;
for (const row of file) {
  if (row.length > max_size) {
    max_size = row.length;
  }
}
const input = file.map((row) => row.padEnd(max_size, " ")).slice(
  0,
  -1,
);
const operations = file[file.length - 1].split(" ").filter((cel) =>
  cel.length > 0
);
const parsed: number[][] = Array.from({ length: operations.length }, () => []);
let target = 0;
for (let i = 0; i < max_size; i++) {
  let new_num = "";
  for (const row of input) {
    new_num += row[i] === " " ? "" : row[i];
  }
  const num = Number(new_num);
  if (num === 0) {
    target += 1;
  } else {
    parsed[target].push(num);
  }
}
const res = parsed.map((list, i) => {
  return list.reduce((acc, num) => {
    if (operations[i] === "*") {
      return acc * num;
    }
    return acc + num;
  }, operations[i] === "*" ? 1 : 0);
});
console.log(res.reduce((acc, num) => acc + num, 0));
