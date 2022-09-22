package org.buildfest2022.qe;

import com.mongodb.AutoEncryptionSettings;
import com.mongodb.ConnectionString;
import com.mongodb.MongoClientSettings;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoCollection;
import java.util.*;
import java.util.HashMap;
import java.util.Map;
import org.bson.BsonArray;
import org.bson.BsonBinary;
import org.bson.BsonBinarySubType;
import org.bson.BsonDocument;
import org.bson.BsonString;
import org.bson.Document;
import org.bson.types.Binary;

/*
 * Insert and find a document in search.documents
 */
public class InsertEncryptedDocument {

  public static void main(String[] args) throws Exception {

    Map<String, String> credentials = Credentials.getCredentials();
    String encryptedDbName = "search";
    String encryptedCollName = "documents";
    String encryptedNameSpace = encryptedDbName + "." + encryptedCollName;

    // start-key-vault
    String keyVaultDb = "encryption";
    String keyVaultColl = "__keyVault";
    String keyVaultNamespace = keyVaultDb + "." + keyVaultColl;
    // end-key-vault

    String connectionString = credentials.get("MONGODB_URI");

    // start-kmsproviders
    Map<String, Map<String, Object>> kmsProviders = new HashMap<String, Map<String, Object>>();
    Map<String, Object> providerDetails = new HashMap<>();
    providerDetails.put("key", credentials.get("LOCAL_KEY_BASE64"));
    kmsProviders.put("local", providerDetails);
    // end-kmsproviders

    // start-schema
    MongoClient regularClient = MongoClients.create(connectionString);

    MongoCollection<Document> keyVaultClient =
        regularClient.getDatabase(keyVaultDb).getCollection(keyVaultColl);

    BsonBinary dataKeyId1 =
        new BsonBinary(
            BsonBinarySubType.UUID_STANDARD,
            keyVaultClient.find().first().get("_id", Binary.class).getData());

    // create the Encrypted Fields Map.
    Map<String, BsonDocument> encryptedFieldsMap = new HashMap<String, BsonDocument>();
    {
      encryptedFieldsMap.put(
          "search.documents",
          new BsonDocument()
              .append(
                  "fields",
                  new BsonArray(
                      Arrays.asList(
                          new BsonDocument()
                              .append("keyId", dataKeyId1)
                              .append("path", new BsonString("body"))
                              .append("bsonType", new BsonString("string"))))));

      encryptedFieldsMap.put(
          "search.lemmas",
          new BsonDocument()
              .append(
                  "fields",
                  new BsonArray(
                      Arrays.asList(
                          new BsonDocument()
                              .append("keyId", dataKeyId1)
                              .append("path", new BsonString("lemma"))
                              .append("bsonType", new BsonString("string"))
                              .append(
                                  "queries",
                                  new BsonDocument()
                                      .append("queryType", new BsonString("equality")))))));
    }

    // end-schema

    // start-extra-options
    Map<String, Object> extraOptions = new HashMap<String, Object>();
    extraOptions.put("cryptSharedLibPath", credentials.get("SHARED_LIB_PATH"));
    // end-extra-options

    // start-client
    MongoClientSettings clientSettings =
        MongoClientSettings.builder()
            .applyConnectionString(new ConnectionString(connectionString))
            .autoEncryptionSettings(
                AutoEncryptionSettings.builder()
                    .keyVaultNamespace(keyVaultNamespace)
                    .kmsProviders(kmsProviders)
                    .encryptedFieldsMap(encryptedFieldsMap)
                    .extraOptions(extraOptions)
                    .build())
            .build();
    MongoClient mongoClientSecure = MongoClients.create(clientSettings);
    // end-client

    // start-insert
    Document doc = new Document("body", "foo");
    mongoClientSecure.getDatabase(encryptedDbName).getCollection(encryptedCollName).insertOne(doc);
    // end-insert

    // start-find
    System.out.println("Finding a document with regular (non-encrypted) client.");
    Document docRegular =
        regularClient.getDatabase(encryptedDbName).getCollection(encryptedCollName).find().first();
    System.out.println(docRegular.toJson());
    System.out.println("Finding a document with encrypted client, searching on an encrypted field");
    Document docSecure =
        mongoClientSecure
            .getDatabase(encryptedDbName)
            .getCollection(encryptedCollName)
            .find()
            .first();
    System.out.println(docSecure.toJson());
    // end-find

    mongoClientSecure.close();
    regularClient.close();
  }
}
