<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Student as Student;
use \MongoDB\BSON\ObjectId as ObjectId;


class Students {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->students->find();
    }

    static function getByName(string $name) {
        return \SchoolApp\Repository\Mongo::getDatabase()->students->find(["name" => $name]);
    }

    static function insertOne(Student $student) {
        return \SchoolApp\Repository\Mongo::getDatabase()->students->insertOne($student->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Mongo::getDatabase()->students->findOne(['_id' => $id]);
    }

}

?>
