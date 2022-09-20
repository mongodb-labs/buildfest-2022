package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;

public interface SecretRepository {
    @NonNull
    Publisher<Secret> list();

    Mono<Boolean> save(@NonNull @NotNull @Valid Secret secret);

    Mono<Boolean> delete(@NonNull @NotNull String name);
}
