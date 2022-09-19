FROM php:8.1.10-buster

COPY --from=composer/composer /usr/bin/composer /usr/bin/composer

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y git libzip-dev zip libssl-dev pkg-config && docker-php-ext-install zip

RUN pecl install mongodb

RUN docker-php-ext-enable --ini-name zz-mongodb.ini mongodb

RUN mkdir -p /school_app/src

COPY ./composer.json /school_app

VOLUME ["/school_app/src"]

WORKDIR /school_app

RUN composer update

EXPOSE 80

CMD ["php", "-S", "0.0.0.0:80", "-t", "/school_app/src", "/school_app/src/index.php"]
