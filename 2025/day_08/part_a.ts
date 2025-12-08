import * as fd from "node:fs";

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, {
  encoding: "utf-8",
})
  .split("\n")
  .filter((row) => row.length > 0)
  .map((row) => row.split(",").map(Number) as [number, number, number]);

const distance = (
  [x1, y1, z1]: [number, number, number],
  [x2, y2, z2]: [number, number, number],
) => {
  const x = Math.pow(x1 - x2, 2);
  const y = Math.pow(y1 - y2, 2);
  const z = Math.pow(z1 - z2, 2);
  return x + y + z;
};

const distances = [];

for (let i = 0; i < file.length - 1; i++) {
  for (let j = i + 1; j < file.length; j++) {
    distances.push(
      [i, j, distance(file[i], file[j])],
    );
  }
}
distances.sort((a, b) => {
  return a[2] - b[2];
});
const grups: number[][] = [];
for (let i = 0; i < 1000; i++) {
  const next_small = distances[i];
  let grup_0;
  let grup_1;
  for (
    let g = 0;
    g < grups.length && (grup_0 === undefined || grup_1 === undefined);
    g++
  ) {
    if (grups[g].includes(next_small[0])) {
      grup_0 = g;
    }
    if (grups[g].includes(next_small[1])) {
      grup_1 = g;
    }
  }
  if (grup_0 === undefined && grup_1 === undefined) {
    grups.push([next_small[0], next_small[1]]);
  } else if (grup_0 !== undefined && grup_1 === undefined) {
    grups[grup_0].push(next_small[1]);
  } else if (grup_0 === undefined && grup_1 !== undefined) {
    grups[grup_1].push(next_small[0]);
  } else if (
    grup_0 !== undefined && grup_1 !== undefined && grup_0 !== grup_1
  ) {
    grups[grup_0] = grups[grup_0].concat(grups[grup_1]);
    grups.splice(grup_1, 1);
  }
}
const res = grups.map((g) => g.length).toSorted((a, b) => b - a).slice(0, 3)
  .reduce((acc, c) => c * acc, 1);
console.log(res);
