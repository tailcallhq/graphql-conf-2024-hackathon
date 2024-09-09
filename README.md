# 🚀 GraphQL Conf Hackathon 2024 | Tailcall

## Overview

Welcome to the Tailcall GraphQL Hackathon 2024, where you will implement a GraphQL server that resolves data from upstream REST APIs! Your mission is to implement the GraphQL schema provided below, efficiently resolving queries while ensuring high performance and correctness.

**Challenge**: Build a GraphQL server that:

- Resolves data from upstream REST APIs.
- Implements the provided GraphQL schema.
- Handles real-world performance scenarios (e.g., multiple requests, nested resolvers).
- The fastest and most reliable implementation wins the top prize!

## Challenge Details

### Objective

Participants must implement a GraphQL API that resolves data from an upstream REST API, provided by the organizers. The data must be resolved according to the predefined schema, and the server must perform well under stress.

### Predefined Schema

The GraphQL schema that you need to implement is specified in the [schema.graphql](./schema.graphql) in the root of this repository.

### Data Source (Upstream REST API)

Your GraphQL server will need to fetch data from the upstream REST API at:

**Base URL**: `http://localhost:3000`

Endpoints:

- /posts (returns a list of posts)
- /posts/:id (returns a post by ID)
- /users (returns a list of users)
- /users/:id (returns a user by ID)
- /users?id=1&id=2&id=3 (returns multiple users with ids in query params)

The structure of the REST API responses will match the GraphQL schema fields.

### GraphQL server

Your GraphQL server should start on url `http://localhost:8000/graphql` and serve `POST` Graphql requests on it.

### Rules

- Participant's implementation should follow stated [objective](#objective)
- The solution should be provided as pull-request to this repo from participant's fork
- The pull-request should contain only file additions inside `/projects/${participant_name}` without changes in the other repo files or other participants code
- The solution could be implemented in any language or framework or using specific tools within the scope of the licence granted by used tools. The only prohibition is  the use of the [tailcall](https://github.com/tailcallhq/tailcall/) tool
- The solution should contain all source code and setup that is required to understand how the solution was achieved and how to run it
- Cooperation on single solution is acceptable, but only the author of the pr will be eligible to win the prize
- In case of the multiple solutions with identical code will be candidates for prize only the solution that was added first will be eligible for prize

## Getting Started

1. Fork this repository
2. Clone the repository locally or run the codespace of your choice
3. Add new folder to `./projects` folder with your username. Copy the `/template` folder content from the repository root to your folder to populate required files.
4. Add the code of the implementation inside the folder
	- you could use any language or tool by your choice that allows you to create the required GraphQL server. Just make sure the solution could be replicated in Github Actions environment
	- follow requirements from [Challenge Details](#challenge-details)
	- use the `schema.graphql` file from the root of the repo. Feel free to copy the file to your folder and change it the way you needed to work properly, but don't change the structure of types
5. Add `run.sh` file that installs required tools and runs the server
	- the script is running on [Github Hosted runner](https://docs.github.com/en/actions/using-github-hosted-runners/using-github-hosted-runners/about-github-hosted-runners). List of available tools and packages could be found [here](https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2404-Readme.md)
	- first add installation and build steps for required tools and code. E.g. `npm i` or `cargo build --release`
	- add steps to start the server. E.g. `npm start` or `cargo run --release`
	- make sure the script is marked as executable `chmod +x run.sh`
6. Make sure your code is working and handles GraphQL requests
7. Commit and push changes to your fork
8. Create a pull request from your fork into original repository

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
