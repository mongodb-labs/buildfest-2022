<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Course as Course;
use \SchoolApp\Repository\Mongo as Mongo;
use \MongoDB\BSON\ObjectId as ObjectId;

class Courses {
    static function index(?string $name) {
        if ($name == null) {
            $courses = \SchoolApp\Repository\Courses::getAll();
        } else {
            $courses = \SchoolApp\Repository\Courses::getByName($name);
        }
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
            $courseId = \SchoolApp\Repository\Courses::insertOne(Course::make($_POST), $session);
            $teacherId = new ObjectId($_POST['teacher']);
            $studentIds = [];
            foreach ($_POST['students'] as $student) {
                array_push($studentIds, new ObjectId($student));
            }
            \SchoolApp\Repository\Teachers::addCourse($teacherId, $courseId, $session);
            foreach ($studentIds as $studentId) {
                \SchoolApp\Repository\Students::addCourse($studentId, $courseId, $session);
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
