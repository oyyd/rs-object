const { loadBinding } = require('@node-rs/helper')

const bindings = loadBinding(__dirname, 'rs-object', 'rs-object')

const File = bindings.createFileClass()

module.exports = {
  File,
}
