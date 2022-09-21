<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

$db = \SchoolApp\Repository\Mongo::getInstance();
$db->students->drop();

$db->students->insertOne(["name" => "Aisha Vaughan" ] );
$db->students->insertOne( [ "name" => "Greta Diaz" ] );
$db->students->insertOne( [ "name" => "Pamela Melton" ] );
$db->students->insertOne( [ "name" => "Alden Gilmore" ] );
$db->students->insertOne( [ "name" => "Cornelius Patel" ] );
$db->students->insertOne( [ "name" => "Jimena Watts" ] );
$db->students->insertOne( [ "name" => "Matteo Arias" ] );
$db->students->insertOne( [ "name" => "Shiloh Ingram" ] );
$db->students->insertOne( [ "name" => "Dustin Holmes" ] );
$db->students->insertOne( [ "name" => "Elisabeth Rowland" ] );
$db->students->insertOne( [ "name" => "Krish Gibbs" ] );
$db->students->insertOne( [ "name" => "Lily Contreras" ] );
$db->students->insertOne( [ "name" => "Karley Huang" ] );
$db->students->insertOne( [ "name" => "Conor Davenport" ] );
$db->students->insertOne( [ "name" => "Keon Moreno" ] );
$db->students->insertOne( [ "name" => "Jameson Stokes" ] );
$db->students->insertOne( [ "name" => "Elijah Schroeder" ] );
$db->students->insertOne( [ "name" => "Tara Griffith" ] );
$db->students->insertOne( [ "name" => "Jenna Hinton" ] );
$db->students->insertOne( [ "name" => "Marley Kerr" ] );
$db->students->insertOne( [ "name" => "Emerson Wolf" ] );
$db->students->insertOne( [ "name" => "Mattie Gillespie" ] );
$db->students->insertOne( [ "name" => "Kamren Roth" ] );
$db->students->insertOne( [ "name" => "Laurel Hubbard" ] );
$db->students->insertOne( [ "name" => "Cristopher Blackwell" ] );
$db->students->insertOne( [ "name" => "Alannah Green" ] );
$db->students->insertOne( [ "name" => "Erica Nichols" ] );
$db->students->insertOne( [ "name" => "Scarlett Lambert" ] );
$db->students->insertOne( [ "name" => "Maurice Fuller" ] );
$db->students->insertOne( [ "name" => "Paris Buchanan" ] );
$db->students->insertOne( [ "name" => "Jordan Porter" ] );
$db->students->insertOne( [ "name" => "Dalia Petersen" ] );
$db->students->insertOne( [ "name" => "Liam Eaton" ] );
$db->students->insertOne( [ "name" => "Rogelio Harrison" ] );
$db->students->insertOne( [ "name" => "Houston Ortiz" ] );
$db->students->insertOne( [ "name" => "Kimberly Berry" ] );
$db->students->insertOne( [ "name" => "Caitlin Price" ] );
$db->students->insertOne( [ "name" => "Caleb Navarro" ] );
$db->students->insertOne( [ "name" => "Xander Moran" ] );
$db->students->insertOne( [ "name" => "Carley Zamora" ] );
$db->students->insertOne( [ "name" => "Danika Barajas" ] );
$db->students->insertOne( [ "name" => "Jaidyn Livingston" ] );
$db->students->insertOne( [ "name" => "Kaleb Farrell" ] );
$db->students->insertOne( [ "name" => "Alec Evans" ] );
$db->students->insertOne( [ "name" => "Nathalie Shaw" ] );
$db->students->insertOne( [ "name" => "Louis Underwood" ] );
$db->students->insertOne( [ "name" => "Raymond Wolf" ] );
$db->students->insertOne( [ "name" => "Valentina Mcdonald" ] );
$db->students->insertOne( [ "name" => "Anya Ewing" ] );
$db->students->insertOne( [ "name" => "Nancy Hines" ] );


$db->teachers->drop();
$db->teachers->insertOne( [ "name" => "Micaela Mendoza" ] );
$db->teachers->insertOne( [ "name" => "Jadyn Valdez" ] );
$db->teachers->insertOne( [ "name" => "Jeremy Mcintosh" ] );
$db->teachers->insertOne( [ "name" => "Anne Higgins" ] );
$db->teachers->insertOne( [ "name" => "Mya Mcdonald" ] );

?>
