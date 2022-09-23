<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Student as Student;
use \MongoDB\BSON\ObjectId as ObjectId;
use \SchoolApp\Repository\Mongo as Mongo;

class Students {
    static function index(?string $name) {
        if ($name == null) {
            $students = \SchoolApp\Repository\Students::getAll();
        } else {
            $students = \SchoolApp\Repository\Students::getByName($name);
        }
        \SchoolApp\View\Students::index($students);
    }

    static function new() {
        \SchoolApp\View\Students::new();
    }

    static function create(Student $student) {
        \SchoolApp\Repository\Students::insertOne($student);
        header("Location: /students", TRUE, 301);
    }
    static function show(string $student) {
        $objectId = new ObjectId($student);
        $student = \SchoolApp\Repository\Students::findOne($objectId);
        \SchoolApp\View\Students::show($student);
    }

    static function delete($studentId) {
        $session = Mongo::getClient()->startSession();
        try {
            $session->startTransaction();
            $id = new ObjectId($studentId);
            \SchoolApp\Repository\Students::delete($id, $session);
            \SchoolApp\Repository\Courses::removeStudent($id, $session);
            $session->commitTransaction();
        } catch (Exception $e) {
            $session->abortTransaction();
            throw $e;
        } finally {
            $session->endSession();
        }


        header("Location: /students", TRUE, 301);
    }
}

?>
