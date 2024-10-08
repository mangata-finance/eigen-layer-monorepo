import { Client } from 'redis-om'
import { transactionSchema } from '../model/Transaction.js'
import { getTimeseriesUrl } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'

const client = new Client()
await client.open(getTimeseriesUrl())

const transactionRepository = client.fetchRepository(transactionSchema)

try {
  await transactionRepository.createIndex()
} catch (error) {
  if (error.message.includes('Index already exists')) {
    logger.log({
      level: 'info',
      message: 'Index already exists, skipping creation.',
    })
  } else {
    throw error
  }
}

export { transactionRepository }
