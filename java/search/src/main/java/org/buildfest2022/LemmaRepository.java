package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import java.util.List;
import javax.validation.Valid;
import javax.validation.constraints.NotNull;
import org.bson.types.ObjectId;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

public interface LemmaRepository {
  @NonNull
  Publisher<Lemma> list();

  Mono<Boolean> save(@NonNull @NotNull @Valid Lemma lemma);

  Mono<Boolean> upsertAll(
      @NonNull @NotNull List<String> lemmas, @NonNull @NotNull ObjectId documentId);
}
