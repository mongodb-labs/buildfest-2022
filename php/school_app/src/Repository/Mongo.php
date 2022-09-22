<?php namespace SchoolApp\Repository;

class Mongo {

    private \MongoDB\Client $client;

    private string $databaseName;

    private function __construct() {
        if ($_ENV['USE_ENCRYPTION'] == '1') {
            $this->databaseName = $_ENV['DATABASE'] . '_encrypted';
            $localKey = new \MongoDB\BSON\Binary(
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                \MongoDB\BSON\Binary::TYPE_GENERIC
            );
            $dataKeyId1 = new \MongoDB\BSON\Binary(base64_decode("mV8hPid+SmO2W37P25wUaQ=="), \MongoDB\BSON\Binary::TYPE_UUID);
            $autoEncryptionOpts = [
                'keyVaultNamespace' => 'encryption.__keyVault',
                'kmsProviders' => ['local' => ['key' => $localKey]],
                'encryptedFieldsMap' => [
                    "$this->databaseName.students" => [
                        'fields' => [
                            [
                                'path' => 'name',
                                'bsonType' => 'string',
                                'keyId' => $dataKeyId1,
                                'queries' => ['queryType' => 'equality'],
                            ],
                        ],
                    ],
                    "$this->databaseName.teachers" => [
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
        } else {
            $this->databaseName = $_ENV['DATABASE'];
            $this->client = new \MongoDB\Client($_ENV['MONGODB_URI']);
        }
    }

    static function getInstance() {
        static $instance;
        if ($instance == null) {
            \MongoDB\Driver\Monitoring\addSubscriber(new EventSubscriber());
            $instance = new Mongo();
        }
        return $instance;
    }


    static function getClient(): \MongoDB\Client {
        return self::getInstance()->client;
    }


    static function getDatabase(): \MongoDB\Database {
        return self::getClient()->selectDatabase(self::getInstance()->databaseName);
    }
}
