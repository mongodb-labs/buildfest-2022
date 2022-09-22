package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import javax.validation.constraints.NotBlank;

@Introspected
public class Search {
  @NonNull @NotBlank private final String query;

  @Creator
  public Search(@NonNull String query) {
    this.query = query;
  }

  @NonNull
  public String getQuery() {
    return this.query;
  }
}
