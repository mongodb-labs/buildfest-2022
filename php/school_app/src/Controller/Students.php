<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Student as Student;
use \MongoDB\BSON\ObjectId as ObjectId; 

class Students {
    static function index() {
        $students = \SchoolApp\Repository\Students::getAll();
        \SchoolApp\View\Students::index($students);
    }
    static function create(Student $student) {
        \SchoolApp\Repository\Students::insertOne($student);
        return "successful insert!";
    }
    static function show(string $student) {
        $objectId = new ObjectId($student);
        $found_student = \SchoolApp\Repository\Students::findOne($objectId);
        \SchoolApp\View\Students::index([$found_student]); 
    }
}

?>
