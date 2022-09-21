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
        header("Access-Control-Allow-Origin: *");
        header('Access-Control-Allow-Origin: *');
        header('Access-Control-Allow-Headers: Content-Type, X-Requested-With');
        header('Access-Control-Allow-Methods: POST, GET, OPTIONS, DELETE, PUT');
        header('Access-Control-Max-Age: 600');
        header('Content-Type: application/json; charset=utf-8');
        $arr = array('success' => true);
        echo json_encode($arr);
    }
    static function show(string $grade) {
        $objectId = new ObjectId($grade);
        $found_grade = \SchoolApp\Repository\Grades::findOne($objectId);
        \SchoolApp\View\Grades::index([$found_grade]); 
    }
}

?>
