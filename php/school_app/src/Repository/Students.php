<?php namespace SchoolApp\Repository;

use Exception;
use \SchoolApp\Model\Student as Student;
use \SchoolApp\Model\Course as Course;
use \MongoDB\BSON\ObjectId as ObjectId;


class Students {

    /**
     * @throws Exception
     */
    static function getAll(): \Iterator
    {
        // Using a transaction just for fun here.
        $session = Mongo::getClient()->startSession();
        try {
            $session->startTransaction();
            $results = Mongo::getDatabase()->selectCollection('students')->find([], ['session' => $session]);
            $session->commitTransaction();
            return $results;
        } catch (Exception $e) {
            $session->abortTransaction();
            throw $e;
        } finally {
            $session->endSession();
        }
    }

    static function getByName(string $name, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('students')->find(
            ["name" => $name],
            $opts
        );
    }

    static function insertOne(Student $student) {
        return Mongo::getDatabase()->selectCollection('students')->insertOne($student->get());
    }

    static function findOne(ObjectId $id, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        $student = Student::make(
            Mongo::getDatabase()->selectCollection('students')->findOne(['_id' => $id])
        );
        $student->courses = array_map(
            function ($doc) { return Course::make($doc); },
            iterator_to_array(\SchoolApp\Repository\Courses::getByIds($student->courseIds))
        );
        return $student;
    }

    static function getByIds(array $ids, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->students->find(['_id' => ['$in' => $ids]]);
    }

    static function addCourse(ObjectId $id, ObjectId $courseId, $session = null) {
        $student = Students::findOne($id, $session);
        if ($student == null) {
            return false;
        }
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->selectCollection('students')->updateOne(
            ['_id' => $student->_id],
            ['$addToSet' => ['courses' => $courseId]],
            $opts
        );
    }

    static function delete(ObjectId $id, $session = null) {
        $opts = [];
        if ($session != null) {
            $opts['session'] = $session;
        }
        return Mongo::getDatabase()->students->deleteOne(
            ['_id' =>  $id],
            $opts
        );
    }
}
