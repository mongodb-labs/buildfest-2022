package org.buildfest2022;

import static io.micronaut.http.HttpStatus.*;

import io.micronaut.core.annotation.NonNull;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Delete;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import javax.validation.Valid;
import javax.validation.constraints.NotNull;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

@Controller("/secrets")
public class SecretController {
  private final SecretRepository secretService;

  SecretController(SecretRepository secretService) {
    this.secretService = secretService;
  }

  @Get
  Publisher<Secret> list() {
    return secretService.list();
  }

  @Post
  Mono<HttpStatus> save(@NonNull @NotNull @Valid Secret secret) {
    return secretService.save(secret).map(added -> added ? CREATED : CONFLICT);
  }

  @Delete
  Mono<HttpStatus> delete(@NonNull @NotNull String name) {
    return secretService.delete(name).map(deleted -> deleted ? OK : CONFLICT);
  }
}
