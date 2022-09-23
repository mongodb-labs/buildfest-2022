<?php namespace SchoolApp\Model;

use \MongoDB\BSON\ObjectId as ObjectId;

class Teacher {

    public ?ObjectId $_id;

    public string $name;

    public array $courseIds;

    public array $courses;

    public function __construct(string $name, $courseIds = [], ?ObjectId $_id = null){
        $this->name = $name;
        $this->courseIds = $courseIds;
        $this->_id = $_id;
    }

    public static function make($map) {
        $id = null;
        if ( ($map['_id'] ?? null) != null) {
            $id = new ObjectId($map['_id']);
        }
        $courseIds = [];
        foreach ( ($map['courses'] ?? []) as $course ) {
            array_push($courseIds, new ObjectId($course));
        }
        return new self(
            $map['name'],
            $courseIds,
            $id
        );
    }

    function get(): array {
        $result = [
            'name' => $this->name,
        ];
        if ($this->_id != null) {
            $result['_id'] = $this->_id;
        }
        return $result;
    }
}

?>
