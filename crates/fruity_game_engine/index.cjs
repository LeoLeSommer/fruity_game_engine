var bundle = undefined

function setBundle(val) {
  bundle = val
  return bundle
}

function getBundle() {
  return bundle
}

module.exports.setBundle = setBundle
module.exports.getBundle = getBundle

module.exports.World = function World(...args) {
  return getBundle().World(...args)
}