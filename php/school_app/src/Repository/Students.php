<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Person as Student;
use \MongoDB\BSON\ObjectId as ObjectId;


class Students {
    static function getAll() {
        return \SchoolApp\Repository\Database::getInstance()->students->find();
    }

    static function insertOne(Student $student) {
        return \SchoolApp\Repository\Database::getInstance()->students->insertOne($student->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Database::getInstance()->students->findOne(['_id' => $id]);
    }

}

?>
