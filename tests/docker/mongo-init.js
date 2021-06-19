conn = Mongo("mongodb://root:example@localhost:27017/admin"); // Connect to Mongo Instance
db = conn.getDB("warp_crud") // Connect to database

db.people.insertMany([
    {"fname": "Nobby", "lname": "Nobbs", "timestamp": ISODate()},
    {"fname": "Fred", "lname": "Colon", "timestamp": ISODate()},
    {"fname": "Carrot", "lname": "Ironfounderson", "timestamp": ISODate()}
]);