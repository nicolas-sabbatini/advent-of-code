import * as fd from "node:fs";

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, { encoding: "utf-8" }).split("\n").map((
  row,
) => row.split("")).filter((row) => row.length > 0);

let res = 0;
for (const ys in file) {
  const y = Number(ys);
  for (const xs in file[y]) {
    const x = Number(xs);
    if (file[y][x] == ".") {
      continue;
    }
    let neighbor = 0;
    for (let x_off = -1; x_off <= 1; x_off += 1) {
      for (let y_off = -1; y_off <= 1; y_off += 1) {
        if (
          x + x_off < 0 || x + x_off >= file[y].length || y + y_off < 0 ||
          y + y_off >= file.length || (x_off == 0 && y_off == 0)
        ) {
          continue;
        }
        if (
          file[y + y_off][x + x_off] === "@"
        ) {
          neighbor += 1;
        }
      }
    }
    if (neighbor < 4) {
      res += 1;
    }
  }
}
console.log(res);
