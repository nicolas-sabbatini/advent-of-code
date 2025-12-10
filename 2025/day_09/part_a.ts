import * as fd from "node:fs";

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, {
  encoding: "utf-8",
})
  .split("\n")
  .filter((row) => row.length > 0)
  .map((row) => row.split(",").map(Number) as [number, number]);

const calculateArea = (a: [number, number], b: [number, number]) => {
  return (Math.abs(a[0] - b[0]) + 1) * (Math.abs(a[1] - b[1]) + 1);
};

const areas = [];

for (let i = 0; i < file.length - 1; i++) {
  for (let j = i + 1; j < file.length; j++) {
    areas.push(calculateArea(file[i], file[j]));
  }
}
areas.sort((a, b) => b - a);
console.log(areas[0]);
