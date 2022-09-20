package org.buildfest2022;

import io.micronaut.http.HttpStatus;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestInstance;
import org.junit.jupiter.api.Timeout;

import org.bson.BsonDocument;

import static io.micronaut.http.HttpStatus.CREATED;
import static org.junit.jupiter.api.Assertions.*;

@MicronautTest
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
public class SecretControllerTest {
  @Inject SecretClient secretClient;

  // Inject a sync MongoClient for testing.
  @Inject com.mongodb.client.MongoClient unencryptedClient;

  @Test
  @Timeout(120)
  void secretsEndpointInteractsWithMongo() {
    // Clear the collection first.
    {
      var database = unencryptedClient.getDatabase("search");
      var collection = database.getCollection("secrets");
      collection.drop();
    }

    {
      HttpStatus status = secretClient.save(new Secret("Secret 1", "1"));
      assertEquals(CREATED, status);
    }

    // Create an unencrypted MongoClient to verify the data is encrypted.
    {
      var database = unencryptedClient.getDatabase("search");
      var collection = database.getCollection("secrets", BsonDocument.class);
      var doc = collection.find().first();
      assertTrue(doc.isBinary("body"));
    }
  }
}
