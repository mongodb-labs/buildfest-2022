<?php namespace SchoolApp\View;

class Teachers {
    public static function index($teachers) {
        require "templates/teachers/index.php";
    }

    public static function new() {
        require "templates/teachers/new.php";
    }

    public static function show($teacher) {
        require "templates/teachers/show.php";
    }
}

?>
