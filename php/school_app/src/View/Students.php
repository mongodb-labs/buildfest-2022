<?php namespace SchoolApp\View;

class Students {
    public static function index($students) {
        require "templates/students/index.php";
    }

    public static function new() {
        require "templates/students/new.php";
    }
}

?>
