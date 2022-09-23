<?php require __DIR__ . "/../header.php" ?>

<table class="table">
  <tbody>
    <tr>
      <th scope="row">Name</th>
      <td><?= $student->name ?></td>
    </tr>
    <tr>
      <th scope="row">Courses</th>
      <td>
        <?php foreach (($student->courses ?? []) as $course) : ?>
          <p><?= $course->name ?></p>
        <?php endforeach ?>
      </td>
    </tr>
  </tbody>
</table>

<?php require __DIR__ . "/../footer.php" ?>
