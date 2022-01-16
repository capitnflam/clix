import { clix } from './index.js'

const scenario = clix('super 2')
  .expect(['Santa', 'New Year'])
  .input()
  .expectError('Failed')

console.log('From native', clix('ls -la'));