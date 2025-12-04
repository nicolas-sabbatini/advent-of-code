import * as fd from "node:fs";

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, { encoding: "utf-8" })
  .split("\n")
  .map((row) =>
    row.split("")
      .map((cell): number => {
        if (cell == ".") {
          return 0;
        }
        return 1;
      })
  )
  .filter((row) => row.length > 0);

const remove = (iteration: number) => {
  let res = 0;
  for (const ys in file) {
    const y = Number(ys);
    for (const xs in file[y]) {
      const x = Number(xs);
      if (file[y][x] < iteration) {
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
            file[y + y_off][x + x_off] >= iteration
          ) {
            neighbor += 1;
          }
        }
      }
      if (neighbor < 4) {
        res += 1;
      } else {
        file[y][x] = iteration + 1;
      }
    }
  }
  return res;
};

let acc = 0;
for (
  let i = 1, removed_rols = remove(i);
  removed_rols !== 0;
  i++, removed_rols = remove(i)
) {
  acc += removed_rols;
}

console.log(acc);
