<?php require __DIR__ . "/../header.php" ?>

<table class="table">
  <tbody>
    <tr>
      <th scope="row">Name</th>
      <td><?= $course->name ?></td>
    </tr>
    <tr>
      <th scope="row">Description</th>
      <td><?= $course->description ?></td>
    </tr>
    <tr>
      <th scope="row">Teacher</th>
      <td><?= $course->teacher->name ?></td>
    </tr>
    <tr>
      <th scope="row">Students</th>
      <td>
        <?php foreach ($course->students as $student) : ?>
          <p><?= $student->name ?></p>
        <?php endforeach ?>
      </td>
    </tr>
  </tbody>
</table>

<?php require __DIR__ . "/../footer.php" ?>
