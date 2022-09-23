<?php require __DIR__ . "/../header.php" ?>

<div class="row">
  <div class="col col-5">
    <a class="btn btn-primary btn-sm" href="/courses/new" role="button">Create Course</a>
  </div>
  <div class="col">
    <form class="row text-center" role="form" method="GET" action="/courses">
    <div class="col">
    <input type="text" class="form-control" placeholder="Filter by name" id="name" name="name">
    </div>
    <div class="col">
    <button type="submit" class="btn btn-primary btn-sm">Filter</button>
    <a class="btn btn-danger btn-sm" href="/courses">Clear filter</a>
    </div>
    </form>
  </div>
</div>
<hr>

<table class="table">
    <thead>
    <tr>
      <th scope="col">Name</th>
      <th scope="col">Description</th>
      <th scope="col">&nbsp;</th>
    </tr>
  </thead>
  <tbody>

    <?php foreach ($courses as $course) : ?>
      <tr>
          <td><?= $course["name"] ?></td>
          <td><?= $course["description"] ?></td>
          <td>
            <a class="btn btn-primary btn-sm" href="/courses/<?= $course["_id"] ?>" role="button">Show</a>
          </td>
      </tr>
    <?php endforeach ?>
  </tbody>
</table>

<?php require __DIR__ . "/../footer.php" ?>
