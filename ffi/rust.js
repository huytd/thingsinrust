const ffi = require('ffi');
const ref = require('ref');
const array = require('ref-array');

let StringArray = array('string');

let lib = ffi.Library('./rsfetch/target/debug/librsfetch.dylib', {
  'fetch_array': ['string', [StringArray, 'int']],
});

let urls = new StringArray([
    "http://google.com",
    "https://huytd.github.io/404.md"
]);

//let result = lib.tf(urls, urls.length);
let result = lib.fetch_array(urls, urls.length);

console.log(result);
