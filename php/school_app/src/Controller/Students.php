<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Student as Student;
use \MongoDB\BSON\ObjectId as ObjectId; 

class Students {
    static function index() {
        $students = \SchoolApp\Repository\Students::getAll();
        \SchoolApp\View\Students::index($students);
    }
    static function form() {
        \SchoolApp\View\Students::form();
    }
    static function create(Student $student) {
        \SchoolApp\Repository\Students::insertOne($student);
        $arr = array('success' => true);
        echo json_encode($arr);
    }
    static function show(string $student) {
        $objectId = new ObjectId($student);
        $found_student = \SchoolApp\Repository\Students::findOne($objectId);
        \SchoolApp\View\Students::index([$found_student]); 
    }
}

?>
