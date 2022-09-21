<?php require __DIR__ . "/../header.php" ?>

<form action="/courses" method="POST">
    <div class="row mb-3">
        <label for="name" class="col-sm-2 col-form-label">Name:</label>
        <div class="col-sm-10">
            <input type="text" id="name" name="name" class="form-control">
        </div>
    </div>
    <div class="row mb-3">
        <label for="description" class="col-sm-2 col-form-label">Description:</label>
        <div class="col-sm-10">
            <input type="text" id="description" name="description" class="form-control">
        </div>
    </div>
    <div class="row mb-3">
        <label for="teacher" class="col-sm-2 col-form-label">Teacher:</label>
        <div class="col-sm-10">
            <select class="form-select" name="teacher">
                <?php foreach ($teachers as $teacher) : ?>
                <option value="<?= $teacher["name"] ?>"><?= $teacher["name"] ?></option>
                <?php endforeach ?>
            </select>
        </div>
    </div>
    <div class="row mb-3">
        <label for="students" class="col-sm-2 col-form-label">Students:</label>
        <div class="col-sm-10">
            <select class="form-select" name="students[]" multiple>
                <?php foreach ($students as $student) : ?>
                <option value="<?= $student["name"] ?>"><?= $student["name"] ?></option>
                <?php endforeach ?>
            </select>
        </div>
    </div>
    <button type="submit" class="btn btn-primary">Submit</button>
</form>

<?php require __DIR__ . "/../footer.php" ?>
