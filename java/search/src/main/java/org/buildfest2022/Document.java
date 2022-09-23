package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import org.bson.codecs.pojo.annotations.BsonCreator;
import org.bson.codecs.pojo.annotations.BsonProperty;

import javax.validation.constraints.NotBlank;

@Introspected
public class Document {
  @NonNull
  @NotBlank
  @BsonProperty("body")
  private final String body;

  @NonNull
  @NotBlank
  @BsonProperty("url")
  private final String url;

  @Creator
  @BsonCreator
  public Document(@NonNull @BsonProperty("body") String body, @NonNull @BsonProperty("url") String url) {
    this.body = body;
    this.url = url;
  }

  @NonNull
  public String getBody() {
    return body;
  }

  @NonNull
  public String getUrl() {
    return url;
  }
}
