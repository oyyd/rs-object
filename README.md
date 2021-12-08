# @node-rs/object

Node.js bindings to Rust [object](https://github.com/gimli-rs/object).

Currently, only a small set of APIs are exported.

## Installation

```n
npm i @node-rs/object
```

## Example

```js
const fs = require('fs')
const { File } = require('@node-rs/object')

const file = new File(fs.readFileSync(process.execPath))

console.log(file.symbols()[0])
// {
//   index: 0,
//   name: '__ZN4node10AsyncHooks18push_async_contextEddN2v85LocalINS1_6ObjectEEE',
//   address: 4294986000,
//   kind: 'Text'
// }
```

## API

See [index.d.ts](./index.d.ts)
