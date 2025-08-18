
import { sortBy, uniq, debounce, groupBy, sumBy } from 'lodash-es';
import { format, addDays, differenceInDays } from 'date-fns';
import { pipe, map, filter, reduce } from 'ramda';

// Dynamically import remote components
export async function loadRemoteComponents() {
  try {
    const [Button, utils, dateUtils, functionalUtils] = await Promise.all([
      import('remote/Button'),
      import('remote/utils'),
      import('remote/dateUtils'),
      import('remote/functionalUtils')
    ]);

    console.log('Remote components loaded successfully!');
    return { 
      Button: Button.default, 
      utils,
      dateUtils,
      functionalUtils
    };
  } catch (error) {
    console.error('Failed to load remote components:', error);
    return null;
  }
}

// Demo data for host app with dates
const items = [
  { name: 'Apple', category: 'fruit', price: 1.5, purchaseDate: new Date('2024-01-15') },
  { name: 'Banana', category: 'fruit', price: 0.8, purchaseDate: new Date('2024-01-16') },
  { name: 'Carrot', category: 'vegetable', price: 1.2, purchaseDate: new Date('2024-01-17') },
  { name: 'Broccoli', category: 'vegetable', price: 2.0, purchaseDate: new Date('2024-01-18') },
  { name: 'Orange', category: 'fruit', price: 1.3, purchaseDate: new Date('2024-01-19') },
  { name: 'Tomato', category: 'vegetable', price: 1.7, purchaseDate: new Date('2024-01-20') },
];

async function initializeApp() {
  const app = document.getElementById('app');
  
  // Use lodash-es in host app
  const sortedItems = sortBy(items, 'price');
  const categories = uniq(items.map(item => item.category));
  const groupedByCategory = groupBy(items, 'category');
  const totalByCategory = Object.entries(groupedByCategory).map(([cat, items]) => ({
    category: cat,
    total: sumBy(items, 'price')
  }));
  
  // Use date-fns in host app
  const today = new Date();
  const nextWeek = addDays(today, 7);
  const formattedToday = format(today, 'PPP');
  const daysFromPurchase = items.map(item => ({
    name: item.name,
    daysAgo: differenceInDays(today, item.purchaseDate)
  }));
  
  // Use ramda in host app - functional pipeline to get expensive items
  const getExpensiveItems = pipe(
    filter(item => item.price > 1.2),
    map(item => ({ ...item, priceWithTax: item.price * 1.1 })),
    reduce((acc, item) => acc + item.priceWithTax, 0)
  );
  const expensiveTotal = getExpensiveItems(items);
  
  app.innerHTML = `
    <div style="padding: 20px; font-family: Arial, sans-serif;">
      <h1>Host App - Module Federation with Shared Modules</h1>
      
      <div style="margin-bottom: 30px; border: 2px solid #007acc; padding: 15px; border-radius: 8px;">
        <h2>📦 Shared Modules Usage in Host</h2>
        
        <div style="background: #f0f8ff; padding: 10px; margin-bottom: 15px; border-radius: 4px;">
          <h3>lodash-es Functions:</h3>
          <p><strong>Items sorted by price:</strong></p>
          <ul>
            ${sortedItems.map(item => `
              <li>${item.name} - $${item.price.toFixed(2)}</li>
            `).join('')}
          </ul>
          <p><strong>Categories (uniq):</strong> ${categories.join(', ')}</p>
          <p><strong>Total by category (groupBy + sumBy):</strong></p>
          <ul>
            ${totalByCategory.map(cat => `
              <li>${cat.category}: $${cat.total.toFixed(2)}</li>
            `).join('')}
          </ul>
        </div>
        
        <div style="background: #f0fff0; padding: 10px; margin-bottom: 15px; border-radius: 4px;">
          <h3>date-fns Functions:</h3>
          <p><strong>Today's date:</strong> ${formattedToday}</p>
          <p><strong>Next week:</strong> ${format(nextWeek, 'PPP')}</p>
          <p><strong>Days since purchase:</strong></p>
          <ul>
            ${daysFromPurchase.map(item => `
              <li>${item.name}: ${item.daysAgo} days ago</li>
            `).join('')}
          </ul>
        </div>
        
        <div style="background: #fff0f5; padding: 10px; margin-bottom: 15px; border-radius: 4px;">
          <h3>ramda Functions (Functional Pipeline):</h3>
          <p><strong>Total for items > $1.20 (with 10% tax):</strong> $${expensiveTotal.toFixed(2)}</p>
          <p style="font-size: 0.9em; color: #666;">
            Pipeline: filter(price > 1.2) → map(add tax) → reduce(sum)
          </p>
        </div>
      </div>

      <div id="remote-components">
        <h2>Loading Remote Components...</h2>
      </div>
    </div>
  `;

  // Load and use remote components
  const remoteComponents = await loadRemoteComponents();
  
  if (remoteComponents) {
    const { Button, utils, dateUtils, functionalUtils } = remoteComponents;
    const remoteContainer = document.getElementById('remote-components');
    
    // Test data for remote utils
    const testUser = { 
      name: 'alice johnson', 
      email: 'alice@example.com', 
      role: 'moderator',
      password: 'secret123',
      internalId: 12345,
      createdAt: new Date('2024-01-01')
    };
    
    const formattedUser = utils.formatUserData(testUser);
    const publicData = utils.pickFields(testUser, ['name', 'email', 'role']);
    
    // Use remote dateUtils
    const userAge = dateUtils.getAgeInDays(testUser.createdAt);
    const schedule = dateUtils.getBusinessDatesInRange(today, nextWeek);
    
    // Use remote functionalUtils
    const processedItems = functionalUtils.processItems(items);
    const stats = functionalUtils.calculateStats(items.map(i => i.price));
    
    remoteContainer.innerHTML = `
      <h2>🌐 Remote Components (Using Same Shared Modules)</h2>
      
      <div style="border: 2px solid #28a745; padding: 15px; border-radius: 8px;">
        <div style="margin-bottom: 20px; background: #e6f4ea; padding: 10px; border-radius: 4px;">
          <h3>Remote Button Component:</h3>
          <div id="button-container"></div>
        </div>
        
        <div style="margin-bottom: 20px; background: #fef7e0; padding: 10px; border-radius: 4px;">
          <h3>Remote Utils (lodash-es):</h3>
          <p><strong>Formatted user (capitalize):</strong></p>
          <pre style="background: #fff; padding: 8px; border: 1px solid #ddd;">${JSON.stringify(formattedUser, null, 2)}</pre>
          <p><strong>Public fields (pick):</strong></p>
          <pre style="background: #fff; padding: 8px; border: 1px solid #ddd;">${JSON.stringify(publicData, null, 2)}</pre>
        </div>
        
        <div style="margin-bottom: 20px; background: #e8f5e9; padding: 10px; border-radius: 4px;">
          <h3>Remote Date Utils (date-fns):</h3>
          <p><strong>User account age:</strong> ${userAge} days</p>
          <p><strong>Business days next week:</strong> ${schedule.length} days</p>
          <ul>
            ${schedule.slice(0, 3).map(date => `
              <li>${format(date, 'EEEE, MMM d')}</li>
            `).join('')}
          </ul>
        </div>
        
        <div style="margin-bottom: 20px; background: #f3e5f5; padding: 10px; border-radius: 4px;">
          <h3>Remote Functional Utils (ramda):</h3>
          <p><strong>Processed items (sorted & transformed):</strong></p>
          <ul>
            ${processedItems.slice(0, 3).map(item => `
              <li>${item.displayName}</li>
            `).join('')}
          </ul>
          <p><strong>Price statistics:</strong></p>
          <ul>
            <li>Average: $${stats.average.toFixed(2)}</li>
            <li>Median: $${stats.median.toFixed(2)}</li>
            <li>Total: $${stats.total.toFixed(2)}</li>
          </ul>
        </div>
      </div>
    `;

    // Create and mount the remote Button component
    const buttonContainer = document.getElementById('button-container');
    if (buttonContainer && Button) {
      const buttonElement = Button({ 
        text: 'hello from remote!', 
        onClick: () => alert('Button clicked! This button came from the remote app.') 
      });
      
      // Since we're not using React, create a simple button manually
      const btn = document.createElement('button');
      btn.textContent = 'Hello From Remote!';
      btn.style.cssText = `
        padding: 10px 20px;
        background-color: #007acc;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
        margin-right: 10px;
      `;
      btn.onclick = () => alert('Button clicked! This button came from the remote app.');
      buttonContainer.appendChild(btn);
      
      // Add a debounced search input to demonstrate shared lodash-es
      const searchContainer = document.createElement('div');
      searchContainer.style.marginTop = '10px';
      searchContainer.innerHTML = `
        <input type="text" id="search-input" placeholder="Type to search (debounced)..." 
               style="padding: 8px; width: 200px; margin-right: 10px;">
        <span id="search-result">Results will appear here...</span>
      `;
      buttonContainer.appendChild(searchContainer);
      
      const searchInput = document.getElementById('search-input');
      const searchResult = document.getElementById('search-result');
      
      // Use debounce from shared lodash-es
      const handleSearch = debounce((value) => {
        const filtered = items.filter(item => 
          item.name.toLowerCase().includes(value.toLowerCase())
        );
        searchResult.textContent = filtered.length 
          ? `Found: ${filtered.map(i => i.name).join(', ')}`
          : 'No items found';
      }, 500);
      
      searchInput.addEventListener('input', (e) => handleSearch(e.target.value));
    }
  }
}

// Demo function that can be tested
export function processItems(items) {
  const sorted = sortBy(items, 'price');
  const categories = uniq(items.map(item => item.category));
  return {
    sorted,
    categories,
    count: items.length
  };
}

// Export functions for testing
export { sortBy, uniq };

// Initialize the app only in browser environment
if (typeof document !== 'undefined') {
  initializeApp();
}