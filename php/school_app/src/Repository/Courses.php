<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Course as Course;
use \MongoDB\BSON\ObjectId as ObjectId;


class Courses {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->courses->find();
    }

    static function insertOne(Course $course) {
        return \SchoolApp\Repository\Mongo::getDatabase()->courses->insertOne($course->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Mongo::getDatabase()->courses->findOne(['_id' => $id]);
    }

}

?>
