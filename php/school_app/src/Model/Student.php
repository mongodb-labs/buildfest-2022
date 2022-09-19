<?php namespace SchoolApp\Model;

class Student {

    public function __construct(string $name, array $courses ){
        $this->name = $name;
        $this->courses = $courses;
    }

    public static function makeWithPost(array $post) {
        return new self("test name", [1,2,3]);
    }

    function getStudent(): array {
        return [
            'name' => $this->name,
            'courses' => $this->courses,
        ];
    }
}

?>