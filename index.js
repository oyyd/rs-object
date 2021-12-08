const { loadBinding } = require('@node-rs/helper')

const bindings = loadBinding(__dirname, 'addon', '@node-rs/object')

const File = bindings.createFileClass()

module.exports = {
  File,
}
