const ffi = require('ffi');
const ref = require('ref');
const array = require('ref-array');

let StringArray = array('string');

let lib = ffi.Library('./rsfetch/target/debug/librsfetch.dylib', {
  'fetch_array': [StringArray, [StringArray, 'int']],
});

let urlArray = Array(1000).fill('http://www.loremipsum.de/downloads/original.txt');
console.log('Jobs: ', urlArray.length);

let urls = new StringArray(urlArray);

let result = lib.fetch_array(urls, urls.length);
result.length = urlArray.length;
