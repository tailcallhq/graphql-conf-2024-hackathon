const { readFileSync } = require('fs');

module.exports = readFileSync(require.resolve('../../../schema.graphql')).toString('utf-8');
