import { logger } from '../utils/common.js';

export class UILibrary {
  createHeader(text: string, tag: string = 'h2'): HTMLElement {
    logger.info(`Creating header element: ${tag}`);
    
    const header = document.createElement(tag);
    header.textContent = text;
    header.className = `ui-header ${tag}`;
    
    return this.styleElement(header, {
      margin: '0 0 1rem 0',
      color: '#1f2937',
    });
  }

  createText(text: string, className?: string): HTMLElement {
    const span = document.createElement('span');
    span.textContent = text;
    
    if (className) {
      span.className = className;
    }
    
    return this.styleElement(span, {
      lineHeight: '1.5',
      color: '#374151',
    });
  }

  createButton(text: string, variant: 'primary' | 'secondary' = 'primary'): HTMLButtonElement {
    logger.info(`Creating button: ${text} (${variant})`);
    
    const button = document.createElement('button');
    button.textContent = text;
    button.type = 'button';
    button.className = `ui-button ui-button--${variant}`;

    const baseStyles = {
      padding: '0.5rem 1rem',
      border: 'none',
      borderRadius: '0.375rem',
      cursor: 'pointer',
      fontSize: '0.875rem',
      fontWeight: '500',
    };

    const variantStyles = variant === 'primary' 
      ? { backgroundColor: '#3b82f6', color: '#ffffff' }
      : { backgroundColor: '#f3f4f6', color: '#374151', border: '1px solid #d1d5db' };

    return this.styleElement(button, { ...baseStyles, ...variantStyles }) as HTMLButtonElement;
  }

  createInput(type: string = 'text', placeholder?: string): HTMLInputElement {
    logger.info(`Creating input element: ${type}`);
    
    const input = document.createElement('input');
    input.type = type;
    input.className = 'ui-input';
    
    if (placeholder) {
      input.placeholder = placeholder;
    }

    return this.styleElement(input, {
      padding: '0.5rem',
      border: '1px solid #d1d5db',
      borderRadius: '0.375rem',
      fontSize: '0.875rem',
    }) as HTMLInputElement;
  }

  styleElement<T extends HTMLElement>(
    element: T, 
    styles: Record<string, string>
  ): T {
    Object.assign(element.style, styles);
    return element;
  }

  createContainer(className?: string): HTMLDivElement {
    const container = document.createElement('div');
    
    if (className) {
      container.className = className;
    }

    return this.styleElement(container, {
      padding: '1rem',
      marginBottom: '1rem',
    });
  }

  createModal(title: string, content: HTMLElement): HTMLElement {
    logger.info(`Creating modal: ${title}`);
    
    const overlay = this.createContainer('modal-overlay');
    const modal = this.createContainer('modal');
    const header = this.createHeader(title, 'h3');
    const closeButton = this.createButton('×', 'secondary');

    modal.appendChild(header);
    modal.appendChild(content);
    modal.appendChild(closeButton);
    overlay.appendChild(modal);

    // Style the modal
    this.styleElement(overlay, {
      position: 'fixed',
      top: '0',
      left: '0',
      width: '100%',
      height: '100%',
      backgroundColor: 'rgba(0, 0, 0, 0.5)',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      zIndex: '1000',
    });

    this.styleElement(modal, {
      backgroundColor: '#ffffff',
      borderRadius: '0.5rem',
      maxWidth: '500px',
      maxHeight: '70vh',
      overflow: 'auto',
    });

    return overlay;
  }
} 