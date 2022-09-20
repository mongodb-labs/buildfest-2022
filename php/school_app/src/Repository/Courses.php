<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Course as Course;
use \MongoDB\BSON\ObjectId as ObjectId;


class Courses {
    static function getAll() {
        return \SchoolApp\Repository\Database::getInstance()->courses->find();
    }

    static function insertOne(Course $course) {
        return \SchoolApp\Repository\Database::getInstance()->courses->insertOne($course->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Database::getInstance()->courses->findOne(['_id' => $id]);
    }

}

?>
