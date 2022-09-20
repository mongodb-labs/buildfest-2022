<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Grade as Grade;
use \MongoDB\BSON\ObjectId as ObjectId; 

class Grades {
    static function index() {
        $grades = \SchoolApp\Repository\Grades::getAll();
        \SchoolApp\View\Grades::index($grades);
    }
    static function create(Grade $grade) {
        \SchoolApp\Repository\Grades::insertOne($grade);
        return "successful insert!";
    }
    static function show(string $grade) {
        $objectId = new ObjectId($grade);
        $found_grade = \SchoolApp\Repository\Grades::findOne($objectId);
        \SchoolApp\View\Grades::index([$found_grade]); 
    }
}

?>
