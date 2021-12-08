const { loadBinding } = require('@rsbind/helper')

const bindings = loadBinding(__dirname, 'addon', '@rsbind/object')

const File = bindings.createFileClass()

module.exports = {
  File,
}
