// Test file for running host app in Node.js environment
import * as _ from 'lodash-es';

console.log('Host App - Testing lodash-es functions');

// Test sortBy
const items = [
  { name: 'Apple', category: 'fruit', price: 1.5 },
  { name: 'Banana', category: 'fruit', price: 0.8 },
  { name: 'Carrot', category: 'vegetable', price: 1.2 },
  { name: 'Broccoli', category: 'vegetable', price: 2.0 },
];

const sorted = _.sortBy(items, 'price');
console.log('Sorted data:', sorted.map(i => `${i.name} ($${i.price})`).join(', '));

// Test uniq
const numbers = [1, 2, 2, 3, 4, 4, 5];
const unique = _.uniq(numbers);
console.log('Unique numbers:', unique.join(', '));

// Test capitalize
const text = 'hello world';
const capitalized = _.capitalize(text);
console.log('Capitalized:', capitalized);

// Test other used functions
const users = [
  { name: 'John', age: 30 },
  { name: 'Jane', age: 25 },
  { name: 'Bob', age: 30 }
];

// Test groupBy
const grouped = _.groupBy(users, 'age');
console.log('Grouped by age:', JSON.stringify(grouped, null, 2));

// Test pick
const user = { name: 'Alice', age: 28, email: 'alice@example.com', password: 'secret' };
const publicData = _.pick(user, ['name', 'email']);
console.log('Picked fields:', JSON.stringify(publicData));

// Test omit
const safeData = _.omit(user, ['password']);
console.log('Omitted fields:', JSON.stringify(safeData));

console.log('Host App tests completed successfully!');