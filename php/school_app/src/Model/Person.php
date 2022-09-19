<?php namespace SchoolApp\Model;

class Person {

    public function __construct(string $name, string $role, array $courses ){
        $this->name = $name;
        $this->role = $role;
        $this->courses = $courses;
    }

    public static function makeWithPost(array $post, string $role) {
        return new self($post['name'], $role, $post['courses']);
    }

    public static function makeStudentWithPost(array $post) {
        return Person::makeWithPost($post, "student");
    }

    public static function makeTeacherWithPost(array $post) {
        return Person::makeWithPost($post, "teacher");
    }

    function get(): array {
        return [
            'name' => $this->name,
            'role' => $this->role,
            'courses' => $this->courses,
        ];
    }
}

?>