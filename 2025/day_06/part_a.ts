import * as fd from "node:fs";

const file_name = Deno.args[0];
const file: string[][] = fd.readFileSync(file_name, { encoding: "utf-8" })
  .split("\n").map((
    row,
  ) => row.split(" ").filter((cel) => cel.length > 0)).filter((row) =>
    row.length > 0
  );

const res = file[0].map((element) => Number(element));
for (let i = 1; i < file.length - 1; i++) {
  for (let j = 0; j < file[i].length; j++) {
    if (file[file.length - 1][j] === "*") {
      res[j] *= Number(file[i][j]);
    } else if (file[file.length - 1][j] === "+") {
      res[j] += Number(file[i][j]);
    } else {
      throw new Error("Unexpected");
    }
  }
}
console.log(
  res.reduce((acc, e) => acc + e, 0),
);
