const { greet, collect_numbers } = require('./pkg/hello_wasm')
const json_obj = require('./data/some-obj.json')
const bench = require('./lib/bench')

console.log("we are here");

const o = {a: 3, b: 'r'};
let str = bench(() => collect_numbers(json_obj));
console.log('ASSSS', str.toString())