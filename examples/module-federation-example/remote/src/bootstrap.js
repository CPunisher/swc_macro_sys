import { groupBy } from 'lodash-es';
import Button from './Button';
import { formatUserData, createDebouncedFunction } from './utils';
import { formatDate, addDaysToDate, compareDates } from './dateUtils';
import { mapData, filterData, processUserData } from './functionalUtils';

// Demo data
const users = [
  { name: 'john doe', email: 'john@example.com', role: 'admin', age: 30, active: true },
  { name: 'jane smith', email: 'jane@example.com', role: 'user', age: 25, active: true },
  { name: 'bob wilson', email: 'bob@example.com', role: 'admin', age: 35, active: false },
];

// Group users by role using lodash-es
const groupedUsers = groupBy(users, 'role');

// Use date-fns functions
const today = new Date();
const futureDate = addDaysToDate(today, 7);
const formattedToday = formatDate(today);
const formattedFuture = formatDate(futureDate);

// Use ramda functions
const activeUsers = processUserData(users);
const userAges = mapData(user => user.age, users);

console.log('Remote app loaded!');
console.log('Grouped users by role:', groupedUsers);
console.log('Today:', formattedToday);
console.log('Future date:', formattedFuture);
console.log('Active users:', activeUsers);
console.log('User ages:', userAges);

// Create a debounced search function
const debouncedSearch = createDebouncedFunction((query) => {
  console.log('Searching for:', query);
}, 500);

// Demo the remote app
if (typeof document !== 'undefined') {
  const app = document.getElementById('app');
  if (app) {
    app.innerHTML = `
      <div>
        <h2>Remote App - Lodash-ES Demo</h2>
        <div>
          <h3>Users grouped by role:</h3>
          <pre>${JSON.stringify(groupedUsers, null, 2)}</pre>
        </div>
        <div>
          <h3>Formatted user data:</h3>
          <pre>${JSON.stringify(users.map(formatUserData), null, 2)}</pre>
        </div>
        <div>
          <h3>Date operations:</h3>
          <p>Today: ${formattedToday}</p>
          <p>Future (+7 days): ${formattedFuture}</p>
        </div>
        <div>
          <h3>Functional operations:</h3>
          <p>Active users: ${JSON.stringify(activeUsers, null, 2)}</p>
          <p>User ages: ${userAges.join(', ')}</p>
        </div>
        <input id="search" placeholder="Type to search (debounced)..." style="padding: 8px; margin: 10px 0; width: 300px;" />
      </div>
    `;

    // Add event listener for debounced search
    const searchInput = document.getElementById('search');
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        debouncedSearch(e.target.value);
      });
    }
  }
}

// Export for testing
export { 
  groupBy, 
  groupedUsers, 
  formatUserData, 
  createDebouncedFunction, 
  users,
  formatDate,
  addDaysToDate,
  compareDates,
  mapData,
  filterData,
  processUserData,
  activeUsers,
  userAges
};