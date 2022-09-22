package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import org.bson.codecs.pojo.annotations.BsonCreator;
import org.bson.codecs.pojo.annotations.BsonProperty;

@Introspected
public class SearchResult {

  @BsonProperty("count")
  private final int count;

  @NonNull
  @BsonProperty("document")
  private final Document document;

  @Creator
  @BsonCreator
  public SearchResult(
      @BsonProperty("count") int count, @BsonProperty("document") Document document) {
    this.count = count;
    this.document = document;
  }

  public int getCount() {
    return count;
  }

  @NonNull
  public Document getDocument() {
    return document;
  }
}
