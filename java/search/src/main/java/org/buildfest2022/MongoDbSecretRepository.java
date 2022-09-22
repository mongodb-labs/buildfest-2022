package org.buildfest2022;

import com.mongodb.reactivestreams.client.MongoClient;
import com.mongodb.reactivestreams.client.MongoCollection;
import io.micronaut.core.annotation.NonNull;
import jakarta.inject.Singleton;
import org.bson.Document;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

@Singleton
public class MongoDbSecretRepository implements SecretRepository {

  private final MongoDbConfiguration mongoConf;

  private final MongoClient mongoClient;

  public MongoDbSecretRepository(MongoDbConfiguration mongoConf, MongoClient mongoClient) {
    this.mongoConf = mongoConf;
    this.mongoClient = mongoClient;
  }

  @Override
  public Publisher<Secret> list() {
    return getCollection().find();
  }

  @Override
  public Mono<Boolean> save(Secret secret) {
    return Mono.from(getCollection().insertOne(secret))
        .map(insertOneResult -> true)
        .onErrorReturn(false);
  }

  @Override
  public Mono<Boolean> delete(String name) {
    Document filter = new Document().append("name", name);
    return Mono.from(getCollection().deleteOne(filter))
        .map(insertOneResult -> true)
        .onErrorReturn(false);
  }

  @NonNull
  private MongoCollection<Secret> getCollection() {
    return mongoClient
        .getDatabase(mongoConf.getName())
        .getCollection(mongoConf.getSecretCollection(), Secret.class);
  }
}
