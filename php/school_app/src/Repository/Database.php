<?php namespace SchoolApp\Repository;

class Database {

    private $client;

    private function __construct() {
        $localKey = new \MongoDB\BSON\Binary(
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            \MongoDB\BSON\Binary::TYPE_GENERIC
        );
        $dataKeyId1 = new \MongoDB\BSON\Binary(base64_decode("mV8hPid+SmO2W37P25wUaQ=="), \MongoDB\BSON\Binary::TYPE_UUID);
        $autoEncryptionOpts = [
            'keyVaultNamespace' => 'encryption.__keyVault',
            'kmsProviders' => ['local' => ['key' => $localKey]],
            'encryptedFieldsMap' => [
                "${_ENV['DATABASE']}.students" => [
                    'fields' => [
                        [
                            'path' => 'name',
                            'bsonType' => 'string',
                            'keyId' => $dataKeyId1,
                            'queries' => ['queryType' => 'equality'],
                        ],
                    ],
                ],
            ],
            'extraOptions' => [
                'cryptSharedLibRequired' => true,
                'cryptSharedLibPath' => '/usr/local/lib/mongo_crypt_v1.so'
            ],
        ];
        $this->client = new \MongoDB\Client(
            $_ENV['MONGODB_URI'],
            [],
            ['autoEncryption' => $autoEncryptionOpts]
        );
    }

    static function getInstance() {
        static $instance;
        if ($instance == null) {
            $instance = (new Database())->client->selectDatabase($_ENV['DATABASE']);
        }
        return $instance;
    }
}

?>
