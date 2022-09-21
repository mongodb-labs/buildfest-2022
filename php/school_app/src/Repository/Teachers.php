<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Teacher as Teacher;
use \MongoDB\BSON\ObjectId as ObjectId;


class Teachers {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getInstance()->teachers->find();
    }

    static function insertOne(Teacher $teacher) {
        return \SchoolApp\Repository\Mongo::getInstance()->teachers->insertOne($teacher->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Mongo::getInstance()->teachers->findOne(['_id' => $id]);
    }

}

?>