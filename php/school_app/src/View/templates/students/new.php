<?php require __DIR__ . "/../header.php" ?>

<form action="/students" method="POST">
    <div class="mb-3">
        <label for="name" class="form-label">First and Last Name:</label>
        <input type="text" id="name" name="name">
    </div>
    <button type="submit" class="btn btn-primary">Submit</button>
</form>

<?php require __DIR__ . "/../footer.php" ?>
