package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import io.micronaut.http.client.annotation.Client;
import java.util.List;
import javax.validation.Valid;
import javax.validation.constraints.NotNull;

@Client("/documents")
public interface DocumentClient {

  @Post
  @NonNull
  HttpStatus save(@NonNull @NotNull @Valid Document document);

  @NonNull
  @Get
  List<Document> findAll();
}
