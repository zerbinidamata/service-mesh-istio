var http = require('http')
var dispatcher = require('httpdispatcher')

var port = process.env.PORT || 8080

dispatcher.onPost(/^\/products/, function (req, res) {
  try {
    productInfo = JSON.parse(req.body)
  } catch (error) {
    res.writeHead(400, {'Content-type': 'application/json'})
    res.end(JSON.stringify({error: 'invalid request body'}))
    return
  }
  var connection = pgConnection()
  connection.connect()
  connection.query(
    'INSERT INTO products (name, price, description) VALUES ($1, $2, $3)', 
    [productInfo.name, productInfo.price, productInfo.description],
    function (error, results, fields) {
      if (error) throw error;
      res.writeHead(500, {'Content-type': 'application/json'})
      res.end(JSON.stringify(results));
    });
  connection.end();
  res.writeHead(200, {'Content-type': 'application/json'})
  res.end(JSON.stringify(productInfo));
})

dispatcher.onGet(/^\/products/, function (req, res) {
  var connection = pgConnection()
  connection.connect()
  connection.query('SELECT * FROM products', function (error, results, fields) {
    if (error) throw error;
    res.writeHead(200, {'Content-type': 'application/json'})
    res.end(JSON.stringify(results));
  });
  connection.end();
})

function pgConnection() {
  var connection = pg.createConnection({
    host: process.env.PG_DB_HOST,
    port: process.env.PG_DB_PORT,
    user: process.env.PG_DB_USER,
    password: process.env.PG_DB_PASSWORD,
    database: process.env.PG_DB_NAME
  })
  return connection;
}

function handleRequest (request, response) {
  try {
    console.log(request.method + ' ' + request.url)
    dispatcher.dispatch(request, response)
  } catch (err) {
    console.log(err)
  }
}

var server = http.createServer(handleRequest)

server.listen(port, function () {
  console.log('Server listening on: http://0.0.0.0:%s', port)
})