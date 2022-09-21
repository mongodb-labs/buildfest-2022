<?php require __DIR__ . "/../header.php" ?>

<table class="table">
    <thead>
    <tr>
      <th scope="col">&nbsp;</th>
      <th scope="col">Name</th>
      <th scope="col">Description</th>
      <th scope="col">Teacher</th>
      <th scope="col">&nbsp;</th>
    </tr>
  </thead>
  <tbody>

    <?php foreach ($courses as $course) : ?>
      <tr>
          <td>&nbsp;</td>
          <td><?= $course["name"] ?></td>
          <td><?= $course["description"] ?></td>
          <td><?= $course["teacher"]["name"] ?></td>
          <td>
              <a class="btn btn-primary btn-sm" href="/courses/<?= $course["_id"] ?>" role="button">Show</a>
              <a class="btn btn-danger btn-sm" href="#" role="button">Delete</a>
          </td>
      </tr>
    <?php endforeach ?>
  </tbody>
</table>

<a class="btn btn-primary" href="/courses/new" role="button">Create course record</a>

<?php require __DIR__ . "/../footer.php" ?>
