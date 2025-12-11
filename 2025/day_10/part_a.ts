import * as fd from "node:fs";

interface Panel {
  target: number[];
  buttons: number[][];
}

interface Response {
  state: number[];
  presses: number;
}

const file_name = Deno.args[0];
const file = fd.readFileSync(file_name, {
  encoding: "utf-8",
})
  .split("\n")
  .filter((row) => row.length > 0)
  .map((row) =>
    row.replaceAll(/[\[\]\{\\(\)}]/gm, "|")
      .split("|")
      .filter((sub) => sub.length > 0 && sub !== " ")
      .map((sub) => sub.split(","))
  );

const panels = file.map((row): Panel => {
  const target = row[0][0].split("").map((char) => char === "#" ? 1 : 0);
  const buttons = row.slice(1, -1).map((btn) =>
    btn.reduce((acc, num) => {
      acc[Number(num)] = 1;
      return acc;
    }, new Array(target.length).fill(0))
  );
  return {
    target,
    buttons,
  };
});

const hash = (state: number[]): string =>
  state.map((s) => s % 2 ? "#" : ".").join("");
const hash_zero = (state: number[]): string => state.map((_) => ".").join("");

const res: Response[] = [];
for (const panel of panels) {
  const cache: Record<string, number> = {};
  cache[hash_zero(panel.target)] = 0;
  const possibilities: Response[] = [{
    state: panel.target.map((_) => 0),
    presses: 0,
  }];
  const target = hash(panel.target);
  next_panel: for (
    let test = possibilities.shift();
    test;
    test = possibilities.shift()
  ) {
    for (const button of panel.buttons) {
      const state = test.state.map((l, i) => l + button[i]);
      const state_hash = hash(state);
      const presses = test.presses + 1;
      if (state_hash === target) {
        res.push({ state, presses });
        break next_panel;
      }
      if (cache[state_hash] === undefined || cache[state_hash] > presses) {
        cache[state_hash] = presses;
        possibilities.push({ state, presses });
      }
    }
  }
}
console.log(res.reduce((acc, p) => p.presses + acc, 0));
