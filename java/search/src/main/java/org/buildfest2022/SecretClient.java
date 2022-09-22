package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.annotation.Delete;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import io.micronaut.http.client.annotation.Client;
import java.util.List;
import javax.validation.Valid;
import javax.validation.constraints.NotNull;

@Client("/secrets")
public interface SecretClient {

  @Post
  @NonNull
  HttpStatus save(@NonNull @NotNull @Valid Secret secret);

  @Delete
  @NonNull
  HttpStatus delete(@NonNull @NotNull String name);

  @NonNull
  @Get
  List<Secret> findAll();
}
