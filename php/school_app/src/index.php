<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

use \SchoolApp\Model\Student as Student;

$router = new \Bramus\Router\Router();

$router->get('/', function() {
    \SchoolApp\Controller\Home::index();
});

$router->get('/students', function() {
    \SchoolApp\Controller\Students::index();
});

$router->get('/students/{studentId}', function($studentId) {
    // NOT IMPLEMENTED YET
    // \SchoolApp\Controller\Students::show($studentId);
});

$router->post('/students', function() {
    \SchoolApp\Controller\Students::create(Student::makeWithPost($_POST));
});

$router->set404(function() {
    header('HTTP/1.1 404 Not Found');
});

$router->run();
?>
