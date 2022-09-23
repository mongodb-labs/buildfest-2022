<?php namespace SchoolApp\Model;

use \MongoDB\BSON\ObjectId as ObjectId;
use \SchoolApp\Model\Teacher as Teacher;
use \SchoolApp\Model\Student as Student;

class Course {

    public ?ObjectId $_id;

    public string $name;

    public ObjectId $teacherId;

    public ?Teacher $teacher;

    public array $studentIds;

    public array $students;

    public function __construct(string $name, ObjectId $teacherId, array $studentIds, string $description, $assignments = [], ?ObjectId $_id = null ){
        $this->_id = $_id;
        $this->name = $name;
        $this->teacherId = $teacherId;
        $this->studentIds = $studentIds;
        $this->description = $description;
        $this->assignments = $assignments;
    }

    public static function make($map) {
        $students = [];
        foreach ($map['students'] as $student) {
            array_push($students, new  ObjectId($student));
        }
        $id = null;
        if ( ($map['_id'] ?? null) != null) {
            $id = new ObjectId($map['_id']);
        }
        return new self(
            $map['name'],
            new ObjectId($map['teacher']),
            $students,
            $map['description'],
            $map['assignments'] ?? [],
            $id
        );
    }

    function get(): array {
        $result = [
            'name' => $this->name,
            'teacher' => $this->teacherId,
            'students' => $this->studentIds,
            'description' => $this->description,
            'assignments' => $this->assignments,
        ];
        if ($this->_id != null) {
            $result['_id'] = $this->id;
        }
        return $result;
    }
}

?>
