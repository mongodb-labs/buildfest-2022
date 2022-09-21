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
 * - Reads master key from file "master-key.txt" in root directory of project, or creates one on a KMS
 * - Locates existing local encryption key from encryption.__keyVault collection, or from a KMS
 * - Prints base 64-encoded value of the data encryption key
 */
public class MakeDataKeyAndCollection {

    public static void main(String[] args) throws Exception {

        Map<String, String> credentials = Credentials.getCredentials();

        // start-kmsproviders
        Map<String, Map<String, Object>> kmsProviders = new HashMap<String, Map<String, Object>>();
        String kmsProvider = "local";
        Map<String, Object> providerDetails = new HashMap<>();
        providerDetails.put("key", credentials.get("LOCAL_KEY_BASE64"));
        kmsProviders.put ("local", providerDetails);
        // end-kmsproviders



        // start-create-index
        String connectionString = credentials.get("MONGODB_URI");
        String keyVaultDb = "encryption";
        String keyVaultColl = "__keyVault";
        MongoClient keyVaultClient = MongoClients.create(connectionString);

        String encryptedDbName = "medicalRecords";
        String encryptedCollName = "patients";

        // Drop the Key Vault Collection in case you created this collection
        // in a previous run of this application.
        keyVaultClient.getDatabase(keyVaultDb).getCollection(keyVaultColl).drop();

        MongoCollection keyVaultCollection = keyVaultClient.getDatabase(keyVaultDb).getCollection(keyVaultColl);
        IndexOptions indexOpts = new IndexOptions().partialFilterExpression(new BsonDocument("keyAltNames", new BsonDocument("$exists", new BsonBoolean(true) ))).unique(true);
        keyVaultCollection.createIndex(new BsonDocument("keyAltNames", new BsonInt32(1)), indexOpts);
        keyVaultClient.close();
        // end-create-index

        // start-create-dek
        String keyVaultNamespace = keyVaultDb + "." + keyVaultColl;
        ClientEncryptionSettings clientEncryptionSettings = ClientEncryptionSettings.builder()
                .keyVaultMongoClientSettings(MongoClientSettings.builder()
                        .applyConnectionString(new ConnectionString(connectionString))
                        .build())
                .keyVaultNamespace(keyVaultNamespace)
                .kmsProviders(kmsProviders)
                .build();
        ClientEncryption clientEncryption = ClientEncryptions.create(clientEncryptionSettings);
        List<String> keyAlts1 = new ArrayList<String>();
        keyAlts1.add("dataKey1");
        BsonBinary dataKeyId1 = clientEncryption.createDataKey(kmsProvider, new DataKeyOptions()
                .keyAltNames(keyAlts1));
        List<String> keyAlts2 = new ArrayList<String>();
        keyAlts2.add("dataKey2");
        BsonBinary dataKeyId2 = clientEncryption.createDataKey(kmsProvider, new DataKeyOptions()
                .keyAltNames(keyAlts2));
        List<String> keyAlts3 = new ArrayList<String>();
        keyAlts3.add("dataKey3");
        BsonBinary dataKeyId3 = clientEncryption.createDataKey(kmsProvider, new DataKeyOptions()
                .keyAltNames(keyAlts3));
        List<String> keyAlts4 = new ArrayList<String>();
        keyAlts4.add("dataKey4");
        BsonBinary dataKeyId4 = clientEncryption.createDataKey(kmsProvider, new DataKeyOptions()
                .keyAltNames(keyAlts4));
        // end-create-dek
        // start-create-enc-collection
        String encryptedNameSpace = encryptedDbName + "." + encryptedCollName;
        BsonDocument encFields = new BsonDocument().append("fields",
                new BsonArray(Arrays.asList(
                        new BsonDocument().append("keyId", dataKeyId1)
                                .append("path", new BsonString("patientId"))
                                .append("bsonType", new BsonString("int"))
                                .append("queries", new BsonDocument().append("queryType", new BsonString("equality"))),
                        new BsonDocument().append("keyId", dataKeyId2)
                                .append("path", new BsonString("medications"))
                                .append("bsonType", new BsonString("array")),
                        new BsonDocument().append("keyId", dataKeyId3)
                                .append("path", new BsonString("patientRecord.ssn"))
                                .append("bsonType", new BsonString("string"))
                                .append("queries", new BsonDocument().append("queryType", new BsonString("equality"))),
                        new BsonDocument().append("keyId", dataKeyId4)
                                .append("path", new BsonString("patientRecord.billing"))
                                .append("bsonType", new BsonString("object"))
                )));
        Map<String, BsonDocument> encryptedFieldsMap = new HashMap<String, BsonDocument>();
        encryptedFieldsMap.put(encryptedNameSpace, encFields);

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
        MongoDatabase encDb = mongoClientSecure.getDatabase(encryptedDbName);
        // Drop the encrypted collection in case you created this collection
        // in a previous run of this application.
        encDb.getCollection(encryptedCollName).drop();
        encDb.createCollection(encryptedCollName);
        // end-create-enc-collection
        System.out.println("Successfully created encrypted collection!");
        mongoClientSecure.close();
        clientEncryption.close();

    }
}