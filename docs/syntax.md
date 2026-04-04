# M0RX Syntax Guide

## Keywords
85 unique keywords covering control flow, functions, memory, async, AI, and backend.

## Data Types
22 types: tiny, short, ant, long, vast, utiny, ushort, uant, ulong, half, dbl, precise, chr, str, txt, bool, nil, list, map, set, tensor, blob

## Operators
42 operators including arithmetic, comparison, logical, bitwise, pipeline |>, null-safe ??

## Comments
```mrx
// Single line comment
/* Multi line
   comment */
```

## String
```mrx
let s: str = "Hello M0RX!"
let multi: txt = "Line 1
Line 2"
```

## List
```mrx
let nums: list = [1, 2, 3, 4, 5]
let first: ant = nums[0]
append(nums, 6)
```

## Map
```mrx
let user: map = {"name": "Alice", "age": 30}
let name: str = user["name"]
```

## Classes
```mrx
class Person {
    let name: str = ""
    let age: ant = 0

    fn greet() -> nil {
        showln(name)
    }
}
```

## Error Handling
```mrx
try {
    let data: str = fread("file.txt")
    showln(data)
} catch err {
    showln(err)
}
```

## Async
```mrx
async fn fetchData(url: str) -> str {
    let data: str = await httpGet(url, {})
    give data
}
```
