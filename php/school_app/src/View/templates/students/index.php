<?php require __DIR__ . "/../header.php" ?>

<p>Student List:</p>
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
    <?php foreach ($students as $student) : ?>
        <tr>
            <td>&nbsp;</td>
            <td><?= $student["name"] ?></td>
            <td>
                <a class="btn btn-primary btn-sm" href="/students/<?= $student["_id"] ?>" role="button">Show</a>
                <a class="btn btn-danger btn-sm" href="#" role="button">Delete</a>
            </td>
    </tr>
    <?php endforeach ?>
    </tbody>
</table>

<a class="btn btn-primary" href="/students/new" role="button">Create Student record</a>

<?php require __DIR__ . "/../footer.php" ?>
