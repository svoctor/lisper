import * as lisper from "lisper-wasm";

var res = lisper.run("(* (+ 2 (/ 50 5)) (+ 1 2))");

console.log("res: " + res);