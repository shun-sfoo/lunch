import React from 'react';
import './App.css';
import { UnauthenticatedApp } from 'unauthenticated-app';
import { useAuth } from 'context/auth-context';
import { AuthenticatedApp } from 'authenticated-app';

function App() {
  const { user } = useAuth();
  return (
    <div className="App">
      {user ? <AuthenticatedApp /> : <UnauthenticatedApp />}
    </div>
  );
}

export default App;
