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

interface Rectangle {
  top: number;
  left: number;
  bottom: number;
  right: number;
}

const rectangle = (i: number, j: number): Rectangle => {
  return {
    top: Math.min(file[i][1], file[j][1]),
    left: Math.min(file[i][0], file[j][0]),
    bottom: Math.max(file[i][1], file[j][1]),
    right: Math.max(file[i][0], file[j][0]),
  };
};

const bunding_box: Rectangle[] = file.reduce((acc, _, i) => {
  if (i === 0) {
    acc.push(rectangle(file.length - 1, i));
  } else {
    acc.push(rectangle(i - 1, i));
  }
  return acc;
}, [] as Rectangle[]);

// Si comparto un borde no lo considero como colisión
const aabb_collision = (a: Rectangle, b: Rectangle) => {
  const left_gap = a.left >= b.right;
  const right_gap = a.right <= b.left;
  const top_gap = a.top >= b.bottom;
  const bottom_gap = a.bottom <= b.top;
  return !(right_gap || left_gap || top_gap ||
    bottom_gap);
};

const areas: [number, Rectangle][] = [];

for (let i = 0; i < file.length - 1; i++) {
  for (let j = i + 1; j < file.length; j++) {
    areas.push([calculateArea(file[i], file[j]), rectangle(i, j)]);
  }
}
areas.sort((a, b) => b[0] - a[0]);
for (const area of areas) {
  let inside = true;
  for (let i = 0; i < bunding_box.length && inside; i++) {
    // Si mi caja se mantiene sin pasar de los bordes sigue siendo váida
    inside = inside && !aabb_collision(area[1], bunding_box[i]);
  }
  if (inside) {
    console.log(area[0]);
    break;
  }
}
