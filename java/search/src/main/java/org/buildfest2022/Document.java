package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import org.bson.codecs.pojo.annotations.BsonCreator;
import org.bson.codecs.pojo.annotations.BsonProperty;
import org.bson.types.ObjectId;

import javax.validation.constraints.NotBlank;

@Introspected
public class Document {

  @BsonProperty("_id")
  private final ObjectId id;

  @NonNull
  @NotBlank
  @BsonProperty("body")
  private final String body;

  @Creator
  @BsonCreator
  public Document(@BsonProperty("_id") ObjectId id, @NonNull @BsonProperty("body") String body) {
    this.id = id;
    this.body = body;
  }

  public Document(@NonNull @BsonProperty("body") String body) {
    this(new ObjectId(), body);
  }

  public ObjectId getId() {
    return id;
  }

  @NonNull
  public String getBody() {
    return body;
  }
}
