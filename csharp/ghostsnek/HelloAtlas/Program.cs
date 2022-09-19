using MongoDB.Driver;

namespace test
{
    class Program
    {
        static void Main(string[] args)
        {
            var connStr = String.Format("mongodb+srv://{0}:{1}@cluster0.gb1qy3e.mongodb.net/?retryWrites=true&w=majority", args[0], args[1]);
            MongoClient dbClient = new MongoClient(connStr);

            var dbList = dbClient.ListDatabases().ToList();

            Console.WriteLine("The list of databases on this server is: ");
            foreach (var db in dbList)
            {
                Console.WriteLine(db);
            }
        }
    }
}