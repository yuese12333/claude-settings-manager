import { canFixJsonStructure, fixJsonStructure } from "./jsonStructureFix";

function assert(cond: boolean, msg: string) {
  if (!cond) throw new Error(msg);
}

{
  const { fixed, changed, notes } = fixJsonStructure('{"a":1');
  assert(changed, "should change");
  assert(JSON.parse(fixed).a === 1, "parse after close brace");
  assert(notes.some((n) => n.includes("括号")), "note braces");
}

{
  const { fixed } = fixJsonStructure('{"a":[1,2');
  assert(JSON.stringify(JSON.parse(fixed)) === JSON.stringify({ a: [1, 2] }), "nested");
}

{
  const { fixed } = fixJsonStructure('{"a":1,}');
  assert(JSON.parse(fixed).a === 1, "trailing comma");
}

{
  const { fixed } = fixJsonStructure('{"a":"hi');
  assert(JSON.parse(fixed).a === "hi", "unclosed string");
}

assert(canFixJsonStructure('{"x":1') === true, "can fix open");
assert(canFixJsonStructure('{"x":1}') === false, "valid no fix");

console.log("jsonStructureFix.check: ok");
