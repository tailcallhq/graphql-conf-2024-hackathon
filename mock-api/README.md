### Dependencies

1. `rust` https://rust-lang.github.io/rustup/installation/index.html


### How to run

In the root directory of the project

`cargo run -p mock-api`


### Routes

* `GET http://127.0.0.1:3000/posts`

  Get all posts

* `GET http://127.0.0.1:3000/posts/1`

  Get specific post

* `GET http://127.0.0.1:3000/users`

  Get all users

* `GET http://127.0.0.1:3000/users/1`

  Get specific user

* `GET http://127.0.0.1:3000/users?id=1&id=2`

  Get specified users