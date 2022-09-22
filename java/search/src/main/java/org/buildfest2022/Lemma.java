package org.buildfest2022;

import io.micronaut.core.annotation.Creator;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.core.annotation.NonNull;
import org.bson.codecs.pojo.annotations.BsonCreator;
import org.bson.codecs.pojo.annotations.BsonProperty;
import org.bson.types.ObjectId;

import javax.validation.constraints.NotBlank;
import java.util.List;

@Introspected
public class Lemma {
  @NonNull
  @NotBlank
  @BsonProperty("lemma")
  private final String lemma;

  @BsonProperty("document_ids")
  private final List<ObjectId> documentIds;

  @Creator
  @BsonCreator
  public Lemma(
      @BsonProperty("lemma") String lemma,
      @BsonProperty("document_ids") List<ObjectId> documentIds) {
    this.lemma = lemma;
    this.documentIds = documentIds;
  }

  @NonNull
  public String getLemma() {
    return lemma;
  }

  public List<ObjectId> getDocumentIds() {
    return documentIds;
  }
}
