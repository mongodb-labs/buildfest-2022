<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

use \SchoolApp\Model\Student as Student;
use \SchoolApp\Model\Teacher as Teacher;
use \SchoolApp\Model\Course as Course;
use \SchoolApp\Model\Grade as Grade;

$router = new \Bramus\Router\Router();

function DecodeJSONPOST(){
    return json_decode(file_get_contents('php://input'), true);
}

$router->get('/', function() {
    \SchoolApp\Controller\Home::index();
});

$router->get('/students', function() {
    \SchoolApp\Controller\Students::index();
});

$router->get('/students/{studentId}', function($studentId) {
    \SchoolApp\Controller\Students::show($studentId);
});

$router->post('/students', function() {
    \SchoolApp\Controller\Students::create(Student::makeWithPost(DecodeJSONPOST()));
});

$router->get('/teachers', function() {
    \SchoolApp\Controller\Teachers::index();
});

$router->get('/teachers/{teacherId}', function($teacherId) {
    \SchoolApp\Controller\Teachers::show($teacherId);
});

$router->post('/teachers', function() {
    \SchoolApp\Controller\Teachers::create(Teacher::makeWithPost(DecodeJSONPOST()));
});

$router->get('/courses', function() {
    \SchoolApp\Controller\Courses::index();
});

$router->get('/courses/{courseId}', function($courseId) {
    \SchoolApp\Controller\Courses::show($courseId);
});

$router->post('/courses', function() {
    \SchoolApp\Controller\Courses::create(Course::makeWithPost(DecodeJSONPOST()));
});

$router->get('/grades', function() {
    \SchoolApp\Controller\Grades::index();
});

$router->get('/grades/{gradeId}', function($gradeId) {
    \SchoolApp\Controller\Grades::show($gradeId);
});

$router->post('/grades', function() {
    \SchoolApp\Controller\Grades::create(Grade::makeWithPost(DecodeJSONPOST()));
});

$router->set404(function() {
    header('HTTP/1.1 404 Not Found');
});

$router->run();
?>
