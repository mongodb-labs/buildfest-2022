<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Student as Student;

class Students {
    static function index() {
        $students = \SchoolApp\Repository\Students::getAll();
        \SchoolApp\View\Students::index($students);
    }
    static function create(Student $student) {
        \SchoolApp\Repository\Students::insertOne(student);
        index();
    }
}

?>
