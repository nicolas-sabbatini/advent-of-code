import * as fd from "node:fs";

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, { encoding: "utf-8" })
  .split("\n").filter((row) => row.length > 0);

const beams = new Array(file[0].length).fill(0);
beams[file[0].indexOf("S")] = 1;
let res = 0;
for (const row of file.slice(1)) {
  let split = row.indexOf("^", 0);
  while (split >= 0) {
    if (beams[split] > 0) {
      beams[split] = 0;
      beams[split + 1] = 1;
      beams[split - 1] = 1;
      res += 1;
    }
    split = row.indexOf("^", split + 1);
  }
}
console.log(res);
