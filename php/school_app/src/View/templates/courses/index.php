<?php require __DIR__ . "/../header.php" ?>

<p>Course List:</p>
<ul>
    <?php foreach ($courses as $course) : ?>
        <li><?= $course["name"]  ?></li>
    <?php endforeach ?>
</ul>

<?php require __DIR__ . "/../footer.php" ?>
