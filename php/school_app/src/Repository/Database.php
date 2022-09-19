<?php namespace SchoolApp\Repository;


class Database {
    private static function mongoClient() {
        static $client;
        if ($client == null)
            $client = new \MongoDB\Client($_ENV['MONGODB_URI']);
        return $client;
    }

    static function getInstance() {
        return Database::mongoClient()->selectDatabase($_ENV['DATABASE']);
    }
}

?>
