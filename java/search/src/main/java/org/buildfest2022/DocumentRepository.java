package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

public interface DocumentRepository {
  @NonNull
  Publisher<Document> list();

  Mono<Boolean> save(@NonNull @NotNull @Valid Document document);

  Publisher<SearchResult> search(@NonNull @NotNull @NotBlank String query);
}
