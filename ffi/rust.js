const ffi = require('ffi');
const ref = require('ref');
const array = require('ref-array');

let StringArray = array('string');

let lib = ffi.Library('./rsfetch/target/debug/librsfetch.dylib', {
  'tf': [StringArray, [StringArray, 'int']],
});

let urls = new StringArray([
    "http://google.com",
    "http://google.com",
    "http://google.com",
]);

//let result = lib.tf(urls, urls.length);
let result = lib.tf(urls, urls.length);
result.length = 2;

console.log(result);
