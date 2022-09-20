<?php require __DIR__ . "/../header.php" ?>

<ul>
    <?php foreach ($teachers as $teacher) : ?>
        <li><?= $teacher["name"]  ?></li>
    <?php endforeach ?>
</ul>

<?php require __DIR__ . "/../footer.php" ?>
