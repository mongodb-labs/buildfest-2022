<?php namespace SchoolApp\Repository;

use \SchoolApp\Model\Student as Student;

class Students {
    static function getAll() {
        return \SchoolApp\Repository\Database::getInstance()->students->find();
    }

    static function insertOne(Student $student) {
        return \SchoolApp\Repository\Database::getInstance()->students->insertOne($student->getStudent());
    }

}

?>
