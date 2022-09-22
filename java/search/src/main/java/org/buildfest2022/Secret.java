package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import javax.validation.constraints.NotBlank;
import org.bson.codecs.pojo.annotations.BsonCreator;
import org.bson.codecs.pojo.annotations.BsonProperty;

@Introspected
public class Secret {
  @NonNull
  @NotBlank
  @BsonProperty("body")
  private final String body;

  @NonNull
  @NotBlank
  @BsonProperty("name")
  private final String name;

  @Creator
  @BsonCreator
  public Secret(
      @NonNull @BsonProperty("body") String body, @NonNull @BsonProperty("name") String name) {
    this.body = body;
    this.name = name;
  }

  @NonNull
  public String getBody() {
    return body;
  }

  @NonNull
  public String getName() {
    return name;
  }
}
