<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Teacher as Teacher;
use \MongoDB\BSON\ObjectId as ObjectId;


class Teachers {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->teachers->find();
    }

    static function insertOne(Teacher $teacher) {
        return \SchoolApp\Repository\Mongo::getDatabase()->teachers->insertOne($teacher->get());
    }

    static function findOne(ObjectId $id) {
        return \SchoolApp\Repository\Mongo::getDatabase()->teachers->findOne(['_id' => $id]);
    }

    static function getByName(string $name, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('teachers')->findOne(
            ["name" => $name],
            $opts
        );
    }

    static function addCourse(string $teacherName, string $courseName, $session = null) {
        $teacher = Teachers::getByName($teacherName, $session);
        if ($teacher == null) {
            return false;
        }
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('teachers')->updateOne(
            ['_id' => $teacher['_id']],
            ['$addToSet' => ['courses' => $courseName]],
            $opts
        );
    }
}

?>
