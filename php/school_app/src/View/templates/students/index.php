<?php require __DIR__ . "/../header.php" ?>

<p>Student List:</p>
<ul>
    <?php foreach ($students as $student) : ?>
        <li><?= $student["name"]  ?></li>
    <?php endforeach ?>
</ul>

<a href="/students/new">Create Student record</a>

<?php require __DIR__ . "/../footer.php" ?>
