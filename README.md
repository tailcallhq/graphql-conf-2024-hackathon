# 🚀 GraphQL Conf Hackathon 2024

Get ready for an adrenaline-pumping, 3-day coding marathon where your mission is clear: **Build the fastest, most resilient GraphQL server** and prove you can **beat Tailcall's performance!**

**When?**

- **Start Date:** 10th September 2024, 8:00 AM PDT
- **End Date:** 12th September 2024, 4:00 PM PDT

This is more than just a competition—it's a race against time, a test of skill, and your chance to make some real money at the conf. The clock is ticking. Are you ready to outcode, outthink, and outperform? Let's do this!

## Getting Started

Support the following GraphQL schema:

```graphql
schema {
  query: Query
}

type Query {
  posts: [Post]
  post(id: Int!): Post
  users: [User]
  user(id: Int!): User
}

type Post {
  id: Int
  userId: Int!
  title: String
  body: String
  user: User
}

type User {
  id: Int
  name: String
  username: String
  email: String
  address: Address
  phone: String
  website: String
  posts: [Post]
}

type Address {
  zipcode: String
  geo: Geo
}

type Geo {
  lat: Float
  lng: Float
}
```

## Technical Requirements

1. All CI tests should pass.
2. Your implementation should be under the `/projects` directory.
3. Should support any query that is supported by the schema.

## And Some More...

- We might add new tests and modify the existing ones to ensure its there is no hardcoding and its a level playing field for all.
- If you questions or doubts about the hackathon, connect with us on [Discord](
https://discord.gg/GJHMeZup8m) or [X](https://x.com/tailcallhq) or the only two people in that bright yellow T-Shirt they'd be glad to say 👋.

Here's a clearer explanation of the scoring calculation for the hackathon:

### Scoring

1. **Test Execution:**

   - For every commit, a set of predefined tests and benchmarks are executed. These tests are located in the `./tests` directory.

2. **Throughput Normalization:**

   - Your performance is measured in terms of requests per second (RPS) for each query.
   - This performance is then compared to Tailcall's RPS for the same query.
   - The comparison is done by dividing your RPS by Tailcall's RPS. This gives a normalized score for each query.

   **Example:**

   - For the `posts-title` query:
     - If your RPS is `100` and Tailcall's RPS is `50`, the normalized score for this query would be `100/50 = 2.0`.

3. **Final Score Calculation:**

   - The normalized scores for all queries are averaged.
   - The final score is this average multiplied by 1000.

   **Example:**

   - Given the following scores:
     | Query | Your RPS | Tailcall RPS | Normalized |
     | ----------------- | -------- | ------------ | ---------- |
     | `posts-nested` | 100 | 50 | 2.0 |
     | `posts-title` | 200 | 350 | 0.8 |
     | `posts-with-user` | 300 | 250 | 1.2 |

   - The average normalized score is `(2.0 + 0.8 + 1.2) / 3 = 1.33`.
   - The final score would be `1.33 * 1000 = 1,333.33`.

## FAQs

**How do I submit my solution?**  
Submit your solution as a pull request (PR) from your forked repo to the main repo.

**What should my PR include?**  
Your PR should only include file additions inside `/projects/${participant_name}`. Don’t change any other files or code belonging to other participants.

**Can I use any language or tools?**  
Yes, you can use any language, framework, or tools as long as they’re within the scope of the licenses. However, using the [tailcall](https://github.com/tailcallhq/tailcall/) tool is not allowed.

**What should be included in the solution?**  
Your solution should include all the source code and setup instructions necessary to understand how you achieved the solution and how to run it.

**Can I work with others on the solution?**  
Yes, you can collaborate, but only the person who submits the PR will be eligible to win the prize.

**What if there are multiple solutions with identical code?**  
Any kind of plagiarism will result in a ban, checkout our [guidelines](https://tailcall.run/docs/contributors/bounty/#identifying-plagiarism) on plagiarism for more.
