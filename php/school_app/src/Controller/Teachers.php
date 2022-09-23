<?php namespace SchoolApp\Controller;

use \SchoolApp\Model\Teacher as Teacher;
use \MongoDB\BSON\ObjectId as ObjectId;

class Teachers {
    static function index(?string $name) {
        if ($name == null) {
            $teachers = \SchoolApp\Repository\Teachers::getAll();
        } else {
            $teachers = \SchoolApp\Repository\Teachers::getByName($name);
        }
        \SchoolApp\View\Teachers::index($teachers);
    }

    static function new() {
        \SchoolApp\View\Teachers::new();
    }

    static function create(Teacher $teacher) {
        \SchoolApp\Repository\Teachers::insertOne($teacher);
        header("Location: /teachers", TRUE, 301);
    }

    static function show(string $teacher) {
        $objectId = new ObjectId($teacher);
        $teacher = \SchoolApp\Repository\Teachers::findOne($objectId);
        \SchoolApp\View\Teachers::show($teacher);
    }
}

?>
