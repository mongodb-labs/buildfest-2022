<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

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
    // NOT IMPLEMENTED YET
    // \SchoolApp\Controller\Students::create($_POST['name']);
});

$router->set404(function() {
    header('HTTP/1.1 404 Not Found');
});

$router->run();
?>
