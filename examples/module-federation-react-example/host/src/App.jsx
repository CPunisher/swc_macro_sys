import React, { Suspense } from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Layout, Spin } from 'antd';
import AppHeader from './components/AppHeader';
import AppSidebar from './components/AppSidebar';
import Dashboard from './pages/Dashboard';
import Analytics from './pages/Analytics';
import Users from './pages/Users';
import Settings from './pages/Settings';
import RemoteShowcase from './pages/RemoteShowcase';

const { Content } = Layout;

const LoadingFallback = () => (
  <div className="loading-container">
    <Spin size="large" tip="Loading..." />
  </div>
);

function App() {
  return (
    <BrowserRouter>
      <Layout className="app-layout">
        <AppSidebar />
        <Layout>
          <AppHeader />
          <Content className="app-content">
            <Suspense fallback={<LoadingFallback />}>
              <Routes>
                <Route path="/" element={<Navigate to="/dashboard" replace />} />
                <Route path="/dashboard" element={<Dashboard />} />
                <Route path="/analytics" element={<Analytics />} />
                <Route path="/users" element={<Users />} />
                <Route path="/remote-components" element={<RemoteShowcase />} />
                <Route path="/settings" element={<Settings />} />
              </Routes>
            </Suspense>
          </Content>
        </Layout>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
