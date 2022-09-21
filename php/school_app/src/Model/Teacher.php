<?php namespace SchoolApp\Model;

class Teacher {

    public function __construct(string $name, array $courses ){
        $this->name = $name;
        $this->courses = $courses;
    }

    public static function makeWithPost(array $post) {
        return new self($post['name'], $post['courses']);
    }

    function get(): array {
        return [
            'name' => $this->name,
            'courses' => $this->courses,
        ];
    }
}

?>