db = new Mongo().getDB("warp_crud");

db.createCollection('people', { capped: false });

db.people.insert([
    {"fname": "Nobby", "lname": "Nobbs", "timestamp": ISODate()},
    {"fname": "Fred", "lname": "Colon", "timestamp": ISODate()},
    {"fname": "Carrot", "lname": "Ironfounderson", "timestamp": ISODate()}
]);