<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Course as Course;
use \SchoolApp\Repository\Mongo as Mongo;
use \MongoDB\BSON\ObjectId as ObjectId;

class Courses {
    static function index() {
        $courses = \SchoolApp\Repository\Courses::getAll();
        \SchoolApp\View\Courses::index($courses);
    }

    static function new() {
        $students = \SchoolApp\Repository\Students::getAll();
        $teachers = \SchoolApp\Repository\Teachers::getAll();
        \SchoolApp\View\Courses::new($students, $teachers);
    }

    static function create() {
        $session = Mongo::getClient()->startSession();
        try {
            $session->startTransaction();
            $course = Course::makeWithPost($_POST);
            \SchoolApp\Repository\Courses::insertOne($course, $session);
            \SchoolApp\Repository\Teachers::addCourse($course->teacher, $course->name, $session);
            foreach ($course->students as $student) {
                \SchoolApp\Repository\Students::addCourse($student, $course->name, $session);
            }
            $session->commitTransaction();
        } catch (Exception $e) {
            $session->abortTransaction();
            throw $e;
        } finally {
            $session->endSession();
        }
        header("Location: /courses", TRUE, 301);
    }

    static function show(string $course) {
        $objectId = new ObjectId($course);
        $found_course = \SchoolApp\Repository\Courses::findOne($objectId);
        \SchoolApp\View\Courses::show($found_course);
    }
}

?>
