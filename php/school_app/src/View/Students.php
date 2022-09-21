<?php namespace SchoolApp\View;

class Students {
    public static function index($students) {
        require "templates/students/index.php";
    }
    public static function form() {
        require "templates/students/form.php";
    }
}

?>
