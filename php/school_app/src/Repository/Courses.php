<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Course as Course;
use \SchoolApp\Model\Teacher as Teacher;
use \SchoolApp\Model\Student as Student;
use \MongoDB\BSON\ObjectId as ObjectId;


class Courses {
    static function getAll() {
        return \SchoolApp\Repository\Mongo::getDatabase()->courses->find();
    }

    static function insertOne(Course $course, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        $result = \SchoolApp\Repository\Mongo::getDatabase()->courses->insertOne(
            $course->get(),
            $opts
        );
        return $result->getInsertedId();
    }

    static function findOne(ObjectId $id) {
        $course = Course::make(\SchoolApp\Repository\Mongo::getDatabase()->courses->findOne(['_id' => $id]));
        $course->teacher = \SchoolApp\Repository\Teachers::findOne($course->teacherId);
        $course->students = array_map(
            function ($doc) { return Student::make($doc); },
            iterator_to_array(\SchoolApp\Repository\Students::getByIds($course->studentIds))
        );
        return $course;
    }

    static function getByName(string $name, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('courses')->find(
            ["name" => $name],
            $opts
        );
    }

    static function getByIds(array $ids, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->courses->find(['_id' => ['$in' => $ids]]);
    }

    static function removeStudent(ObjectId $studentId, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        Mongo::getDatabase()->courses->updateMany(
            [],
            ['$pull' => ['students' => ['$in' => [$studentId]]]],
            $opts
        );
    }
}

?>
