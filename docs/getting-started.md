# Getting Started with M0RX

## Installation
```bash
curl -sSf https://raw.githubusercontent.com/Morx-code-dev/m0rx-lang/main/install.sh | sh
```

## First Program

Create `hello.mrx`:
```mrx
showln("Hello M0RX!")
```

Run it:
```bash
morxc hello.mrx
```

## Variables
```mrx
let name: str = "M0RX"
let age: ant = 1
let score: dbl = 9.5
let active: bool = true
let items: list = [1, 2, 3]
```

## Functions
```mrx
fn add(a: ant, b: ant) -> ant {
    give a + b
}

let result: ant = add(10, 20)
showln(result)
```

## Control Flow
```mrx
if score > 9.0 {
    showln("Excellent!")
} elif score > 7.0 {
    showln("Good!")
} else {
    showln("Keep trying!")
}

each item in items {
    showln(item)
}
```

## Next Steps
- Read [Syntax Guide](syntax.md)
- Explore [Standard Library](stdlib.md)
- Check [Examples](../examples/)
