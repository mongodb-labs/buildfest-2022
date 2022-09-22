package org.buildfest2022;

import com.mongodb.client.model.*;
import com.mongodb.reactivestreams.client.MongoClient;
import com.mongodb.reactivestreams.client.MongoCollection;
import io.micronaut.core.annotation.NonNull;
import jakarta.inject.Singleton;
import org.bson.types.ObjectId;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.List;

@Singleton
public class MongoDbLemmaRepository implements LemmaRepository {

  private final MongoDbConfiguration mongoConf;

  private final MongoClient mongoClient;

  public MongoDbLemmaRepository(MongoDbConfiguration mongoConf, MongoClient mongoClient) {
    this.mongoConf = mongoConf;
    this.mongoClient = mongoClient;
  }

  @Override
  public Publisher<Lemma> list() {
    return getCollection().find();
  }

  @Override
  public Mono<Boolean> save(Lemma lemma) {
    return Mono.from(getCollection().insertOne(lemma))
        .map(insertOneResult -> true)
        .onErrorReturn(false);
  }

  public Mono<Boolean> upsertAll(
      @NonNull @NotNull List<String> lemmas, @NonNull @NotNull ObjectId documentId) {

    List<WriteModel<Lemma>> bulkUpsert = new ArrayList<>();
    for (String lemma : lemmas) {
      bulkUpsert.add(
          new UpdateOneModel<>(
              Filters.eq("lemma", lemma),
              Updates.addToSet("document_ids", documentId),
              new UpdateOptions().upsert(true)));
    }

    return Mono.from(getCollection().bulkWrite(bulkUpsert))
        .map(bulkWriteResult -> true)
        .onErrorReturn(false);
  }

  @NonNull
  private MongoCollection<Lemma> getCollection() {
    return mongoClient
        .getDatabase(mongoConf.getName())
        .getCollection(mongoConf.getLemmaCollection(), Lemma.class);
  }
}

//db.lemmas.updateOne({ lemma: "way" }, { $addToSet: { document_ids: ObjectId("632baf390dac80ee9b2d7149") } }, { upsert: true })