<?php require __DIR__ . "/../header.php" ?>

<p>Grade List:</p>
<ul>
    <?php foreach ($grades as $grade) : ?>
        <li><?= $grade["assignment_name"]  ?></li>
    <?php endforeach ?>
</ul>

<?php require __DIR__ . "/../footer.php" ?>
