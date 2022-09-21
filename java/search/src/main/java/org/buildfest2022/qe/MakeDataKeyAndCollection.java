package org.buildfest2022.qe;

import java.util.*;

import java.util.HashMap;
import java.util.Map;


import org.bson.BsonArray;
import org.bson.BsonBinary;
import org.bson.BsonDocument;
import org.bson.BsonString;
import org.bson.BsonInt32;
import org.bson.BsonBoolean;

import com.mongodb.AutoEncryptionSettings;
import com.mongodb.ClientEncryptionSettings;
import com.mongodb.ConnectionString;
import com.mongodb.MongoClientSettings;

import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import com.mongodb.client.model.IndexOptions;
import com.mongodb.client.model.vault.DataKeyOptions;
import com.mongodb.client.vault.ClientEncryption;
import com.mongodb.client.vault.ClientEncryptions;

/*
 * - Recreates the key vault collection and adds one Data Encryption Key (DEK).
 * - Recreates the documents and lemmas collections with Queryable Encryption enabled.
 * - Prints the Encrypted Fields Map.
 */
public class MakeDataKeyAndCollection {

    public static void main(String[] args) throws Exception {

        Map<String, String> credentials = Credentials.getCredentials();

        // Create KMS providers.
        Map<String, Map<String, Object>> kmsProviders = new HashMap<String, Map<String, Object>>();
        {
            String kmsProvider = "local";
            Map<String, Object> providerDetails = new HashMap<>();
            providerDetails.put("key", credentials.get("LOCAL_KEY_BASE64"));
            kmsProviders.put("local", providerDetails);
        }

        String connectionString = credentials.get("MONGODB_URI");
        String keyVaultDb = "encryption";
        String keyVaultColl = "__keyVault";

        System.out.println ("Recreate Key Vault Collection ... begin");
        {
            MongoClient keyVaultClient = MongoClients.create(connectionString);
            keyVaultClient.getDatabase(keyVaultDb).getCollection(keyVaultColl).drop();

            MongoCollection keyVaultCollection = keyVaultClient.getDatabase(keyVaultDb).getCollection(keyVaultColl);
            IndexOptions indexOpts = new IndexOptions().partialFilterExpression(new BsonDocument("keyAltNames", new BsonDocument("$exists", new BsonBoolean(true) ))).unique(true);
            keyVaultCollection.createIndex(new BsonDocument("keyAltNames", new BsonInt32(1)), indexOpts);
            keyVaultClient.close();
        }
        System.out.println ("Recreate Key Vault Collection ... end");

        System.out.println ("Create Data Encryption Key ... begin");
        String keyVaultNamespace = keyVaultDb + "." + keyVaultColl;
        ClientEncryptionSettings clientEncryptionSettings = ClientEncryptionSettings.builder()
                .keyVaultMongoClientSettings(MongoClientSettings.builder()
                        .applyConnectionString(new ConnectionString(connectionString))
                        .build())
                .keyVaultNamespace(keyVaultNamespace)
                .kmsProviders(kmsProviders)
                .build();
        ClientEncryption clientEncryption = ClientEncryptions.create(clientEncryptionSettings);
        BsonBinary dataKeyId1 = clientEncryption.createDataKey("local", new DataKeyOptions());
        System.out.println ("Create Data Encryption Key ... end");

        // create the Encrypted Fields Map.
        Map<String, BsonDocument> encryptedFieldsMap = new HashMap<String, BsonDocument>();
        {
            encryptedFieldsMap.put("search.documents", new BsonDocument().append("fields",
                    new BsonArray(Arrays.asList(
                            new BsonDocument().append("keyId", dataKeyId1)
                                    .append("path", new BsonString("body"))
                                    .append("bsonType", new BsonString("string"))
                    ))));

            encryptedFieldsMap.put("search.lemmas", new BsonDocument().append("fields",
                    new BsonArray(Arrays.asList(
                            new BsonDocument().append("keyId", dataKeyId1)
                                    .append("path", new BsonString("lemma"))
                                    .append("bsonType", new BsonString("string"))
                                    .append("queries", new BsonDocument().append("queryType", new BsonString("equality")))
                    ))));
        }

        System.out.println ("Drop and create collections in the Encrypted Fields Map ... begin");
        {
            Map<String, Object> extraOptions = new HashMap<String, Object>();
            extraOptions.put("cryptSharedLibPath", credentials.get("SHARED_LIB_PATH"));

            MongoClientSettings clientSettings = MongoClientSettings.builder()
                    .applyConnectionString(new ConnectionString(connectionString))
                    .autoEncryptionSettings(AutoEncryptionSettings.builder()
                            .keyVaultNamespace(keyVaultNamespace)
                            .kmsProviders(kmsProviders)
                            .encryptedFieldsMap(encryptedFieldsMap)
                            .extraOptions(extraOptions)
                            .build())
                    .build();
            MongoClient mongoClientSecure = MongoClients.create(clientSettings);

            for (String name : encryptedFieldsMap.keySet()) {
                String[] parts = name.split("\\.");
                String db = parts[0];
                String coll = parts[1];
                System.out.println ("  creating collection : " + db + "." + coll);
                MongoDatabase encDb = mongoClientSecure.getDatabase(db);
                // Drop the encrypted collection in case you created this collection
                // in a previous run of this application.
                encDb.getCollection(coll).drop();
                encDb.createCollection(coll);
            }

            mongoClientSecure.close();
        }
        System.out.println ("Drop and create collections in the Encrypted Fields Map ... end");

        var b64encoder = Base64.getEncoder();
        var key1IdString = new String(b64encoder.encode (dataKeyId1.asBinary().getData()));
        System.out.println ("created key with id: " + key1IdString);

        clientEncryption.close();

    }
}