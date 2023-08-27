// node 2015/day-12-cheat.js | cargo run --bin 2015-12a

const FILE = require('fs').readFileSync("2015/inputs/12-day.txt", 'utf-8');

const INPUT = JSON.parse(FILE, (key, value) => {
  if (!Array.isArray(value)) return Object.keys(value).map(key => value[key]).indexOf('red') !== -1 ? {} : value;
  return value;
});

console.log(JSON.stringify(INPUT))