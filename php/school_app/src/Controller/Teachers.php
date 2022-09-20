<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Teacher as Teacher;
use \MongoDB\BSON\ObjectId as ObjectId; 

class Teachers {
    static function index() {
        $teachers = \SchoolApp\Repository\Teachers::getAll();
        \SchoolApp\View\Teachers::index($teachers);
    }
    static function create(Teacher $teacher) {
        \SchoolApp\Repository\Teachers::insertOne($teacher);
        return "successful insert!";
    }
    static function show(string $teacher) {
        $objectId = new ObjectId($teacher);
        $found_teacher = \SchoolApp\Repository\Teachers::findOne($objectId);
        \SchoolApp\View\Teachers::index([$found_teacher]); 
    }
}

?>
