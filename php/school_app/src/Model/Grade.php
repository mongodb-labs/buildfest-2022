<?php namespace SchoolApp\Model;

class Grade {

    public function __construct(int $score, string $recipient, string $course, string $assignment_name ){
        $this->score = $score;
        $this->recipient = $recipient;
        $this->course = $course;
        $this->assignment_name = $assignment_name;
    }

    public static function make(array $post) {
        return new self($post['score'], $post['recipient'], $post['course'], $post['assignment_name']);
    }

    function get(): array {
        return [
            'score' => $this->score,
            'recipient' => $this->recipient,
            'course' => $this->course,
            'assignment_name' => $this->assignment_name,
        ];
    }
}

?>
