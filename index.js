const { loadBinding } = require('@node-rs/helper')

const bindings = loadBinding(__dirname, 'package-template', 'rs-object')

const File = bindings.createFileClass()

module.exports = {
  File,
}
