import { describe, it, expect, beforeAll } from 'vitest';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

describe('React Module Federation Integration', () => {
  let hostDistPath;
  let remoteDistPath;

  beforeAll(() => {
    hostDistPath = path.resolve(__dirname, '../../host/dist');
    remoteDistPath = path.resolve(__dirname, '../../remote/dist');
  });

  it('should build host and remote apps', () => {
    expect(fs.existsSync(hostDistPath)).toBe(true);
    expect(fs.existsSync(remoteDistPath)).toBe(true);
  });

  it('should expose React components from remote', () => {
    const remoteEntry = path.join(remoteDistPath, 'remoteEntry.js');
    expect(fs.existsSync(remoteEntry)).toBe(true);
    
    const content = fs.readFileSync(remoteEntry, 'utf8');
    expect(content).toContain('UserCard');
    expect(content).toContain('DataTable');
    expect(content).toContain('ChartWidget');
    expect(content).toContain('FormBuilder');
  });
});
