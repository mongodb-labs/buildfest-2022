package org.buildfest2022;

import edu.stanford.nlp.ling.CoreLabel;
import edu.stanford.nlp.pipeline.CoreDocument;
import edu.stanford.nlp.pipeline.StanfordCoreNLP;
import io.micronaut.core.annotation.NonNull;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import org.bson.types.ObjectId;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Mono;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.List;
import java.util.Properties;

import static io.micronaut.http.HttpStatus.CONFLICT;
import static io.micronaut.http.HttpStatus.CREATED;

@Controller("/documents")
public class DocumentController {
  private final DocumentRepository documentService;
  private final LemmaRepository lemmaService;

  DocumentController(DocumentRepository documentService, LemmaRepository lemmaService) {
    this.documentService = documentService;
    this.lemmaService = lemmaService;
  }

  @Get
  Publisher<Document> list() {
    return documentService.list();
  }

  @Post
  Mono<HttpStatus> save(@NonNull @NotNull @Valid Document document) {
    ObjectId documentId = documentService.save(document).block();
    if (documentId == null) {
      return Mono.just(CONFLICT);
    }

    List<String> lemmas = getLemmas(document.getBody());

    return lemmaService.upsertAll(lemmas, documentId).map(added -> added ? CREATED : CONFLICT);
  }

  private List<String> getLemmas(String body) {
    Properties props = new Properties();
    props.setProperty("annotators", "tokenize,pos,lemma");

    StanfordCoreNLP pipeline = new StanfordCoreNLP(props);
    CoreDocument coreDocument = pipeline.processToCoreDocument(body);

    List<String> lemmas = new ArrayList<>();
    for (CoreLabel tok : coreDocument.tokens()) {
      // Lowercase and remove all non-word characters from the lemma string.
      String lemma = tok.lemma().toLowerCase().replaceAll("\\W", "");
      // If the string is blank or if it's one of the excluded words, skip it.
      if (lemma.isBlank() || excludedWords.contains(lemma)) {
        continue;
      }
      lemmas.add(lemma);
      System.out.println(String.format("%s\t%s", tok.word(), lemma));
    }

    return lemmas;
  }

  private static List<String> excludedWords =
      List.of(
          "a",
          "about",
          "above",
          "after",
          "again",
          "against",
          "all",
          "am",
          "an",
          "and",
          "any",
          "are",
          "as",
          "at",
          "be",
          "because",
          "been",
          "before",
          "being",
          "below",
          "between",
          "both",
          "but",
          "by",
          "could",
          "did",
          "do",
          "does",
          "doing",
          "down",
          "during",
          "each",
          "few",
          "for",
          "from",
          "further",
          "had",
          "has",
          "have",
          "having",
          "he",
          "her",
          "here",
          "hers",
          "herself",
          "him",
          "himself",
          "his",
          "how",
          "i",
          "if",
          "in",
          "into",
          "is",
          "it",
          "its",
          "itself",
          "me",
          "more",
          "most",
          "my",
          "myself",
          "nor",
          "of",
          "on",
          "once",
          "only",
          "or",
          "other",
          "ought",
          "our",
          "ours",
          "ourselves",
          "out",
          "over",
          "own",
          "same",
          "she",
          "should",
          "so",
          "some",
          "such",
          "than",
          "that",
          "the",
          "their",
          "theirs",
          "them",
          "themselves",
          "then",
          "there",
          "these",
          "they",
          "this",
          "those",
          "through",
          "to",
          "too",
          "under",
          "until",
          "up",
          "very",
          "was",
          "we",
          "were",
          "what",
          "when",
          "where",
          "which",
          "while",
          "who",
          "whom",
          "why",
          "with",
          "will",
          "would",
          "you",
          "your",
          "yours",
          "yourself",
          "yourselves");
}
