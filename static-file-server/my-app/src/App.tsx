import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { PathList } from './screen';
import { Button, PageHeader, Upload } from 'antd';
import { UploadOutlined } from '@ant-design/icons';

const apiUrl = process.env.REACT_APP_API_URL;
console.log(apiUrl);

function App() {
  const [list, setList] = useState([]);

  useEffect(() => {
    window.fetch(`${apiUrl}/index_or_content`).then(async (response) => {
      const data = await response.json();
      if (response.ok) {
        setList(data);
      } else {
        Promise.reject(data);
      }
    });
  }, []);

  return (
    <div className="App">
      <PathList list={list} />
    </div>
  );
}

export default App;
