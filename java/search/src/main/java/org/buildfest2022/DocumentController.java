package org.buildfest2022;

import io.micronaut.core.annotation.NonNull;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;

import static io.micronaut.http.HttpStatus.CONFLICT;
import static io.micronaut.http.HttpStatus.CREATED;

@Controller("/documents")
public class DocumentController {
  private final DocumentRepository documentService;

  DocumentController(DocumentRepository documentService) {
    this.documentService = documentService;
  }

  @Get
  Publisher<Document> list() {
    return documentService.list();
  }

  @Post
  Mono<HttpStatus> save(@NonNull @NotNull @Valid Document document) {
    return documentService.save(document).map(added -> added ? CREATED : CONFLICT);
  }
}
