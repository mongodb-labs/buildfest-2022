<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

use \SchoolApp\Model\Student as Student;

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
    // NOT IMPLEMENTED YET
    // \SchoolApp\Controller\Students::show($studentId);
});

$router->post('/students', function() {
    \SchoolApp\Controller\Students::create(Student::makeWithPost(DecodeJSONPOST()));
});

$router->set404(function() {
    header('HTTP/1.1 404 Not Found');
});

$router->run();
?>
