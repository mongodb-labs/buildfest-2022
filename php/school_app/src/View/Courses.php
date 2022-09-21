<?php namespace SchoolApp\View;

class Courses {
    public static function index($courses) {
        require "templates/courses/index.php";
    }

    public static function new($students, $teachers) {
        require "templates/courses/new.php";
    }

    public static function show($course) {
        require "templates/courses/show.php";
    }
}

?>
