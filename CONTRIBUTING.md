### Objective

Participants must implement a GraphQL API that resolves data from an upstream REST API that is deployed on port `3000`.

### Predefined Schema

The GraphQL schema that you need to implement is specified in the [schema.graphql](./schema.graphql) in the root of this repository.

### Data Source (Upstream REST API)

On the CI your GraphQL server will need to fetch data from the upstream REST API at:

**Base URL**: `http://localhost:3000`

### Endpoints

- **GET** `/posts`  
  _Returns a list of posts._

- **GET** `/posts/:id`  
  _Returns a post by ID._

- **GET** `/users`  
  _Returns a list of users._

- **GET** `/users/:id`  
  _Returns a user by ID._

- **GET** `/users?id=1&id=2&id=3`  
  _Returns multiple users with IDs specified in query parameters._

The structure of the REST API responses will match the GraphQL schema fields.

### GraphQL server

Your GraphQL server should start on url `http://localhost:8000/graphql` and serve `POST` Graphql requests on it.

## Getting Started

1. Fork this repository
2. Clone the repository locally or run the codespace of your choice
3. Add new folder to `./projects` folder with your username. Copy the `/template` folder content from the repository root to your folder to populate required files.
4. Add the code of the implementation inside the folder
   - you could use any language or tool by your choice that allows you to create the required GraphQL server. Just make sure the solution could be replicated in Github Actions environment.
   - use the `schema.graphql` file from the root of the repo. Feel free to copy the file to your folder and change it the way you needed to work properly, but don't change the structure of types
5. Add `run.sh` file that installs required tools and runs the server
   - the script is running on [Github Hosted runner](https://docs.github.com/en/actions/using-github-hosted-runners/using-github-hosted-runners/about-github-hosted-runners). List of available tools and packages could be found [here](https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2404-Readme.md)
   - first add installation and build steps for required tools and code. E.g. `npm i` or `cargo build --release`
   - add steps to start the server. E.g. `npm start` or `cargo run --release`
   - make sure the script is marked as executable `chmod +x run.sh`
6. Make sure your code is working and handles GraphQL requests
7. Commit and push changes to your fork
8. Create a pull request from your fork into original repository

### Run mock server locally

To run the mock server locally you need a [Rust toolchain](https://rustup.rs) installed.

To run the mock server in the root of the repo run:

```sh
cargo run -p mock-api
```

The server will start on `http://localhost:3000` and will serve the endpoints mentioned in [data source](#data-source-upstream-rest-api)

### Run test suite locally

To run the whole test suite locally you need a [Rust toolchain](https://rustup.rs) installed.

For the first time you need to build the mock server code (one-time run):

```sh
cargo build -p mock-api
```

After finishing the command you can use following command to run test suite:

```sh
cargo run
```

If you need to run only specific project, specify this project as option with name of the directory of the project:

```sh
cargo run -- --project tailcall
```

## How implementation is checked

1. Build everything that is required to run test environment and custom implementation
2. Start the test environment to validate response: mock server and reference server that is used to test implementation correctness
3. Run correctness tests
4. Run the benchmark
5. Run correctness tests again

### Testing correctness

For testing the correctness repeat next process multiple times:

1. Regenerate mocks on mock-api server
2. For every request in `/tests` directory execute the request to user implementation
3. Execute the same request for reference implementation
4. Compare the results and in case they are mismatch throw an error

### Benchmarking the performance

Ran many requests in parallel to the server with tools like `wrk` or `k6` to collect info about provided RPS and latency
