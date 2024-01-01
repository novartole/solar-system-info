A RESTful web service based on [axum](https://docs.rs/axum/latest/axum/), which provides info about the solar system, and images of planets (thanks to [rust-embed](https://docs.rs/rust-embed/latest/rust_embed/)). The info stores in MongoDB, and [mongodb](https://docs.rs/mongodb/latest/mongodb/) crate is used for interation with the database. Redis is used for cache management (see [redis-rs](https://github.com/redis-rs/redis-rs) crate). 

The servers implements CRUD but all oparation, which may change data, such as POST, PUT and DELETE are allowed only for authorized users. In this project Basic Auth is used for the authentication (see [http-auth-basic](https://github.com/EstebanBorai/http-auth-basic) and [argon2](https://docs.rs/argon2/latest/argon2/)).

This project was inspired by [this](https://habr.com/ru/articles/568856/) article. I re-wrote the business logic in [axum](https://docs.rs/axum/latest/axum/), changed data initialization, wrapped some operation into Basic Auth, and added a few nuances based on my interpritation of the project's goal.

### Endpoints
Open for all users:
- GET: /planets/ - get all planets,
- GET: /planets/:id - get a planet by the id
- GET: /planet/:id/image - get an image of a planet found by the id.
  
Available only for authorized users:
- POST, planet_dto.json: /planets - create a planet based on the json in the body,
- PUT, planet_dto.json: /planets/:id - change a planet according to the id in the path and json in the body,
- DELETE: /planets/:id - delete a planet by the id
  
### Development
If the target machine has pre-installed Redis and MongoDB, then a simple Run command can be used. Requiered ENVs could be put either directly:
```bash
MONGODB_URI=.. REDIS_URI=.. cargo run
```
or via a config file:
```bash
CONFIG_FILE="/path/to/config" cargo run
```

Another option is to use Docker Compose. In this case, .env file should be created in the root folder. This file must contains admin credentials (MONGODB_USERNAME and MONGODB_PASSWORD) for MongoDB. 

*Note that this approach is not safe and was chosen only for simplicity. Using secrets of Docker Compose is more preferable way of doing this.*

Afterwards just run:
```bash
docker compose up
```
