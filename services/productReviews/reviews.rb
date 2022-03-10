require 'webrick'
require 'json'
require 'net/http'
require 'mongo'

class Server < WEBrick::HTTPServlet::AbstractServlet
  def do_GET (req, res)
    begin
      id = Integer(path_parts[-1])
    rescue ArgumentError
      raise 'provide numeric product id'
    end
    reviews = get_product_reviews(id)
    res.body = reviews.to_json
    res['Content-Type'] = 'application/json'
    res.status = 200
  rescue StandardError => e
    res.body = { 'error' => e }.to_json
    res['Content-Type'] = 'application/json'
    res.status = 400
  end

  def do_POST (req, res)
    begin
      id = Integer(path_parts[-1])
    rescue ArgumentError
      raise 'provide numeric product id'
    end
    create_product_review(id, JSON.parse(req.body))
    res['Content-Type'] = 'application/json'
    res.body = { 'success' => true }.to_json
    res.status = 201
  rescue StandardError => e
    res.body = { 'error' => e }.to_json
    res['Content-Type'] = 'application/json'
    res.status = 400
  end

  def get_product_reviews(id)
    client = mongo_client
    product_reviews = client[:products].find(product_id: id)
    client.close
    product_reviews
  end

  def create_product_review(id, review)
    client = mongo_client
    client[:products].insert_one(product_id: id, review: review)
    client.close
  end

  def mongo_client
    mongo_host = ENV.fetch('MONGO_URI') { 'mongodb://localhost:27017' }
    mongo_db = ENV.fetch('MONGO_DATABASE') { 'reviews' }
    # Mongo::Connection.new(mongo_host).db(mongo_db)
    Mongo::Client.new([mongo_host], database: mongo_db)
  end
end

port = ENV.fetch('PORT') || 3000

server = WEBrick::HTTPServer.new(Port: port)

server.mount '/reviews', Server

trap('INT') { server.shutdown }

server.start
