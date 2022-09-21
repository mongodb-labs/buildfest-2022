<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Course as Course;
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
        $course = Course::makeWithPost($_POST);
        \SchoolApp\Repository\Courses::insertOne($course);
        header("Location: /courses", TRUE, 301);
    }

    static function show(string $course) {
        $objectId = new ObjectId($course);
        $found_course = \SchoolApp\Repository\Courses::findOne($objectId);
        \SchoolApp\View\Courses::show($found_course);
    }
}

?>
