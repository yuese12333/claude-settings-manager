/** 简单 JSON 结构修复：尾逗号、未闭合字符串、缺失的 } / ] */

export type StructureFix = {
  fixed: string;
  changed: boolean;
  /** 做了哪些事（短说明） */
  notes: string[];
};

function stripBom(s: string) {
  return s.replace(/^\uFEFF/, "");
}

/** 去掉 `}` / `]` 前的尾逗号 */
function stripTrailingCommas(s: string): { out: string; n: number } {
  let n = 0;
  const out = s.replace(/,(\s*[}\]])/g, (_, rest) => {
    n += 1;
    return rest;
  });
  return { out, n };
}

/**
 * 扫描括号栈；若停在字符串内则补 `"`，再按栈补齐 `}` / `]`。
 * 不尝试改键名、补逗号等复杂错误。
 */
function closeOpenConstructs(s: string): { out: string; notes: string[] } {
  const notes: string[] = [];
  const stack: Array<"}" | "]"> = [];
  let inStr = false;
  let escape = false;

  for (let i = 0; i < s.length; i++) {
    const c = s[i]!;
    if (inStr) {
      if (escape) {
        escape = false;
        continue;
      }
      if (c === "\\") {
        escape = true;
        continue;
      }
      if (c === '"') inStr = false;
      continue;
    }
    if (c === '"') {
      inStr = true;
      continue;
    }
    if (c === "{") stack.push("}");
    else if (c === "[") stack.push("]");
    else if (c === "}" || c === "]") {
      const top = stack[stack.length - 1];
      if (top === c) stack.pop();
    }
  }

  let out = s;
  if (inStr) {
    out += '"';
    notes.push("补全未闭合的字符串引号");
  }
  if (stack.length) {
    const closers = [...stack].reverse().join("");
    out += closers;
    notes.push(`补全未闭合的括号 ${closers}`);
  }
  return { out, notes };
}

export function fixJsonStructure(input: string): StructureFix {
  const original = stripBom(input);
  const notes: string[] = [];
  let s = original;

  const commas = stripTrailingCommas(s);
  s = commas.out;
  if (commas.n) notes.push(`去掉 ${commas.n} 处尾逗号`);

  const closed = closeOpenConstructs(s);
  s = closed.out;
  notes.push(...closed.notes);

  // 若仍无法 parse，再试一轮尾逗号（补括号后偶发）
  const again = stripTrailingCommas(s);
  s = again.out;
  if (again.n) notes.push(`再去掉 ${again.n} 处尾逗号`);

  return {
    fixed: s === original ? input : s,
    changed: s !== original,
    notes,
  };
}

/** 当前文本是否像「可一键修」的结构性错误 */
export function canFixJsonStructure(input: string): boolean {
  const raw = stripBom(input);
  try {
    JSON.parse(raw);
    return false;
  } catch {
    const { changed, fixed } = fixJsonStructure(raw);
    if (!changed) return false;
    try {
      JSON.parse(fixed);
      return true;
    } catch {
      // 有改动但不保证能 parse：仍提供按钮（用户可看 diff 效果）
      return true;
    }
  }
}
