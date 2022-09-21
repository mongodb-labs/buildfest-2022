<?php require __DIR__ . "/../header.php" ?>

<div class="row">
  <div class="col col-5">
    <a class="btn btn-primary btn-sm" href="/students/new" role="button">Create Student record</a>
  </div>
  <div class="col">
    <form class="row text-center" role="form" method="GET" action="/students">
    <div class="col">
    <input type="text" class="form-control" placeholder="Filter by name" id="name" name="name">
    </div>
    <div class="col">
    <button type="submit" class="btn btn-primary btn-sm">Filter</button>
    <a class="btn btn-danger btn-sm" href="/students">Clear filter</a>
    </div>
    </form>
  </div>
</div>
<hr>
<table class="table">
    <thead>
    <tr>
      <th scope="col">Name</th>
      <th scope="col">&nbsp;</th>
    </tr>
  </thead>
  <tbody>
    <?php foreach ($students as $student) : ?>
      <tr>
          <td><?= $student["name"] ?></td>
          <td>
              <a class="btn btn-primary btn-sm" href="/students/<?= $student["_id"] ?>" role="button">Show</a>
              <a class="btn btn-danger btn-sm" href="#" role="button">Delete</a>
          </td>
      </tr>
    <?php endforeach ?>
    </tbody>
</table>



<?php require __DIR__ . "/../footer.php" ?>
