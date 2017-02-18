var buffer = new Buffer("A9018D0002A9058D0102A9088D0202", "hex");
var fs = require('fs');
fs.writeFile('./code.hex', buffer, function(err){ 
  console.log(err); 
});
