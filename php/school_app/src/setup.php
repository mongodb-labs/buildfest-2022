<?php namespace SchoolApp;

require __DIR__ . '/../vendor/autoload.php';

use MongoDB\BSON\Binary;
use MongoDB\Client;

$localKey = new Binary("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", Binary::TYPE_GENERIC);

$encryptionOpts = [
    'keyVaultNamespace' => 'encryption.__keyVault',
    'kmsProviders' => ['local' => ['key' => $localKey]],
];

$client = new Client();
$clientEncryption = $client->createClientEncryption($encryptionOpts);

$dataKeyId1 = $clientEncryption->createDataKey('local');
echo base64_encode((string) $dataKeyId1);
echo "\n----------------------------------\n";
?>
