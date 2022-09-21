<?php require __DIR__ . "/../header.php" ?>

<table class="table">
    <thead>
    <tr>
      <th scope="col">&nbsp;</th>
      <th scope="col">Name</th>
      <th scope="col">&nbsp;</th>
    </tr>
  </thead>
  <tbody>

  </tbody>
    <?php foreach ($teachers as $teacher) : ?>
        <tr>
            <td>&nbsp;</td>
            <td><?= $teacher["name"] ?></td>
            <td>
                <a class="btn btn-primary btn-sm" href="/teachers/<?= $teacher["_id"] ?>" role="button">Show</a>
                <a class="btn btn-danger btn-sm" href="#" role="button">Delete</a>
            </td>
    </tr>
    <?php endforeach ?>
    </tbody>
</table>

<a class="btn btn-primary" href="/teachers/new" role="button">Create Teacher record</a>

<?php require __DIR__ . "/../footer.php" ?>
