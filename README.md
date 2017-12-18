# Rust and PostgreSQL execution test on Heroku

## Create application

```sh
$ heroku create heroku-rust-pg-test
```

## Set Buildpack

```sh
$ heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust.git --app heroku-rust-pg-test
```

## Provisioning Heroku Postgres

```sh
$ heroku addons:create heroku-postgresql:hobby-dev --app heroku-rust-pg-test
```

## Deploy application

```sh
$ heroku git:remote --app heroku-rust-pg-test
$ git push heroku master
```

## Open application in a web browser

```sh
$ heroku open --app heroku-rust-pg-test
```
