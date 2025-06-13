import { logger, debounceClick, formatDate, formatCurrency } from '../utils/common.js';
import { UILibrary } from './ui-library.js';

export class ComponentRenderer {
  private uiLib: UILibrary;

  constructor() {
    this.uiLib = new UILibrary();
  }

  renderUserCard(user: { id: string; name: string; email: string }): HTMLElement {
    logger.info(`Rendering user card for ${user.name}`);
    
    const card = document.createElement('div');
    card.className = 'user-card';
    
    const header = this.uiLib.createHeader(user.name, 'h3');
    const email = this.uiLib.createText(user.email, 'user-email');
    const timestamp = this.uiLib.createText(
      `Created: ${formatDate(new Date())}`,
      'timestamp'
    );

    card.appendChild(header);
    card.appendChild(email);
    card.appendChild(timestamp);

    return card;
  }

  renderPriceDisplay(amount: number, currency?: string): HTMLElement {
    logger.info(`Rendering price display for ${amount}`);
    
    const priceElement = document.createElement('div');
    priceElement.className = 'price-display';
    priceElement.textContent = formatCurrency(amount, currency);
    
    return this.uiLib.styleElement(priceElement, {
      fontSize: '1.2em',
      fontWeight: 'bold',
      color: '#2563eb',
    });
  }

  renderClickableButton(text: string, onClick: () => void): HTMLElement {
    logger.info(`Creating clickable button: ${text}`);
    
    const button = this.uiLib.createButton(text);
    
    // Use debounced click handler
    button.addEventListener('click', () => {
      debounceClick(onClick);
    });

    return button;
  }

  renderDataTable(data: Array<Record<string, any>>): HTMLElement {
    logger.info(`Rendering data table with ${data.length} rows`);
    
    const table = document.createElement('table');
    table.className = 'data-table';

    if (data.length === 0) {
      const emptyRow = document.createElement('tr');
      const emptyCell = document.createElement('td');
      emptyCell.textContent = 'No data available';
      emptyRow.appendChild(emptyCell);
      table.appendChild(emptyRow);
      return table;
    }

    // Create header
    const headerRow = document.createElement('tr');
    Object.keys(data[0]).forEach(key => {
      const th = this.uiLib.createHeader(key, 'th');
      headerRow.appendChild(th);
    });
    table.appendChild(headerRow);

    // Create data rows
    data.forEach(row => {
      const tr = document.createElement('tr');
      Object.values(row).forEach(value => {
        const td = this.uiLib.createText(String(value), 'table-cell');
        tr.appendChild(td);
      });
      table.appendChild(tr);
    });

    return this.uiLib.styleElement(table, {
      width: '100%',
      borderCollapse: 'collapse',
    });
  }
} 