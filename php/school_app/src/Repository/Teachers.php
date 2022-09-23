<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Teacher as Teacher;
use \SchoolApp\Model\Course as Course;
use \MongoDB\BSON\ObjectId as ObjectId;


class Teachers {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->teachers->find();
    }

    static function insertOne(Teacher $teacher) {
        return \SchoolApp\Repository\Mongo::getDatabase()->teachers->insertOne($teacher->get());
    }

    static function findOne(ObjectId $id, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        $teacher = Teacher::make(
            \SchoolApp\Repository\Mongo::getDatabase()->teachers->findOne(
                ['_id' => $id],
                $opts
            )
        );
        $teacher->courses = array_map(
            function ($doc) { return Course::make($doc); },
            iterator_to_array(\SchoolApp\Repository\Courses::getByIds($teacher->courseIds))
        );
        return $teacher;
    }

    static function getByName(string $name, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('teachers')->find(
            ["name" => $name],
            $opts
        );
    }

    static function addCourse(ObjectId $id, ObjectId $courseId, $session = null) {
        $teacher = Teachers::findOne($id, $session);
        if ($teacher == null) {
            return false;
        }
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('teachers')->updateOne(
            ['_id' => $teacher->_id],
            ['$addToSet' => ['courses' => $courseId]],
            $opts
        );
    }
}

?>
