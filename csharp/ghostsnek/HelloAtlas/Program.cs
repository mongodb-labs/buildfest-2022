using MongoDB.Bson;
using MongoDB.Bson.Serialization;
using MongoDB.Driver;

namespace test
{
    class Program
    {
        static void Main(string[] args)
        {
            BsonClassMap.RegisterClassMap<Record>();

            var connStr = String.Format("mongodb+srv://{0}:{1}@cluster0.gb1qy3e.mongodb.net/?retryWrites=true&w=majority", args[0], args[1]);
            MongoClient client = new MongoClient(connStr);

            var dbList = client.ListDatabases().ToList();

            Console.WriteLine("The list of databases on this server is: ");
            foreach (var db in dbList)
            {
                Console.WriteLine(db);
            }

            var coll = client.GetDatabase("buildfest2022").GetCollection<Record>("hello");
            var rec = new Record { name = "Bob" };
            coll.InsertOne(rec);
        }
    }

    class Record
    {
        public ObjectId id { get; set; }
        public string? name { get; set; }
    }
}