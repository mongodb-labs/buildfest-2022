<?php namespace SchoolApp\Model;

class Course {
    public function __construct(string $name, string $teacher, array $students, string $description, array $assignments ){
        $this->name = $name;
        $this->teacher = $teacher;
        $this->students = $students;
        $this->description = $description;
        $this->assignments = $assignments;
    }

    public static function makeWithPost(array $post) {
        return new self($post['name'], $post['teacher'], $post['students'], $post['description'], $post['assignments']);
    }

    function get(): array {
        return [
            'name' => $this->name,
            'teacher' => $this->teacher,
            'students' => $this->students,
            'description' => $this->description,
            'assignments' => $this->assignments,
        ];
    }
}

?>