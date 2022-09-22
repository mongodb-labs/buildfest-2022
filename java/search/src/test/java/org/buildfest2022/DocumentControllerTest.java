package org.buildfest2022;

import static io.micronaut.http.HttpStatus.CREATED;
import static org.junit.jupiter.api.Assertions.*;

import io.micronaut.http.HttpStatus;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestInstance;
import org.junit.jupiter.api.Timeout;

@MicronautTest
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
public class DocumentControllerTest {
  @Inject DocumentClient documentClient;

  @Test
  @Timeout(120)
  void documentsEndpointInteractsWithMongo() {
    List<Document> documents = documentClient.findAll();
    assertTrue(documents.isEmpty());

    HttpStatus status = documentClient.save(new Document("Document body lorem ipsum."));
    assertEquals(CREATED, status);

    documents = documentClient.findAll();
    assertFalse(documents.isEmpty());
    assertEquals("Document body lorem ipsum.", documents.get(0).getBody());
  }
}
