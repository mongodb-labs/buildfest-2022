<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Grade as Grade;
use \MongoDB\BSON\ObjectId as ObjectId;


class Grades {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->grades->find();
    }

    static function insertOne(Grade $grade) {
        return \SchoolApp\Repository\Mongo::getDatabase()->grades->insertOne($grade->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Mongo::getDatabase()->grades->findOne(['_id' => $id]);
    }

}

?>
