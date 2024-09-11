import fastify from 'fastify'
import mercurius from 'mercurius'
import schema from './schema'
import resolvers from './resolvers'
import axiosInstance from './axios'
import DataLoader from 'dataloader'

const server = fastify({
    logger: true
})

server.register(mercurius, {
    schema,
    resolvers,
    context: () => {
        let cache = {};
        let postsCache = {};

        const userDataLoader = new DataLoader(async function (ids: any) {
            const response = await axiosInstance.get('/users', {
                params: { id: ids },

            });
            const users = response.data;
            let obj = {} as any

            users.map((user: any) => {
                obj[user.id] = user;
            })
            return ids.map((id: any) => obj[id]);
        }, { cache: true })

        const postDataLoader = new DataLoader(async function (ids: any) {
            const posts = await Promise.all(
                ids.map(async (id: any) => {
                    try {
                        const response = await axiosInstance.get(`/posts/${id}`,);
                        return response.data;
                    } catch (error: any) {
                        return new Error(`Failed to fetch post with id ${id}: ${error.message}`);
                    }
                })
            );

            return posts;
        }, { cache: true })

        return {
            userDataLoader,
            postDataLoader,
            cache,
            postsCache
        }
    },
    graphiql: true
})

server.get('/ping', async (request, reply) => {
    return 'pong\n'
})

server.get('/testing', async function (req, reply) {
    const query = '{ add(x: 2, y: 2) }'
    return reply.graphql(query)
})

server.listen({ port: 8000 }, (err, address) => {
    if (err) {
        console.error(err)
        process.exit(1)
    }
    console.log(`Server listening at ${address}`)
})