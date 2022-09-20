<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Course as Course;
use \MongoDB\BSON\ObjectId as ObjectId; 

class Courses {
    static function index() {
        $courses = \SchoolApp\Repository\Courses::getAll();
        \SchoolApp\View\Courses::index($courses);
    }
    static function create(Course $course) {
        \SchoolApp\Repository\Courses::insertOne($course);
        return "successful insert!";
    }
    static function show(string $course) {
        $objectId = new ObjectId($course);
        $found_course = \SchoolApp\Repository\Courses::findOne($objectId);
        \SchoolApp\View\Courses::index([$found_course]); 
    }
}

?>
