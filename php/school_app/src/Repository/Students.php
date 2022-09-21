<?php namespace SchoolApp\Repository;

use Exception;
use \SchoolApp\Model\Student as Student;
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

    static function getByName(string $name) {
        return Mongo::getDatabase()->selectCollection('students')->find(["name" => $name]);
    }

    static function insertOne(Student $student) {
        return Mongo::getDatabase()->selectCollection('students')->insertOne($student->get());
    }

    static function findOne(ObjectId $id) {
        return Mongo::getDatabase()->selectCollection('students')->findOne(['_id' => $id]);
    }

}
