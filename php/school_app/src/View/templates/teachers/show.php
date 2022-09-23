<?php require __DIR__ . "/../header.php" ?>

<table class="table">
  <tbody>
    <tr>
      <th scope="row">Name</th>
      <td><?= $teacher->name ?></td>
    </tr>
    <tr>
      <th scope="row">Courses</th>
      <td>
        <?php foreach ($teacher->courses as $course) : ?>
          <p><?= $course->name ?></p>
        <?php endforeach ?>
      </td>
    </tr>
  </tbody>
</table>

<?php require __DIR__ . "/../footer.php" ?>
