const ffi = require('ffi');
const ref = require('ref');
const array = require('ref-array');

let StringArray = array('string');

let lib = ffi.Library('./rsfetch/target/debug/librsfetch.dylib', {
  'tf': ['string', [StringArray, 'int']]
});

let urls = new StringArray([
    "Hello",
    "World"
]);

console.log(lib.tf(urls, urls.length));
