import fastify from 'fastify'
import mercurius from 'mercurius'
import schema from './schema'
import resolvers from './resolvers'

const server = fastify({
    logger: true
})

server.register(mercurius, {
    schema,
    resolvers
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