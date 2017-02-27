const fetch = require('node-fetch');

let jobs = Array(1000).fill(0).reduce((jobs, i) => {
    jobs.push(fetch('http://www.loremipsum.de/downloads/original.txt'));
    return jobs;
}, []);

console.log('Jobs: ', jobs.length);

Promise.all(jobs)
  .then(values => {
    values.map((item) => {
      item.text().then(data => {
        console.log('Result:', data.length);
      });
    });
  })
  .catch(error => {
    console.log('Error:', error);
  });

