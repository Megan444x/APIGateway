import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';

const Post = ({ title, body }) => (
  <div style={{ marginBottom: '20px', border: '1px solid #ddd', padding: '10px', borderRadius: '5px' }}>
    <h2>{title}</h2>
    <p>{body}</p>
  </div>
);

const App = () => {
  const [posts, setPosts] = useState([]);

  useEffect(() => {
    fetch('https://jsonplaceholder.typicode.com/posts')
      .then(response => response.json())
      .then(data => setPosts(data.slice(0, 10)))
      .catch(error => console.error('Error fetching posts:', error));
  }, []);

  return (
    <div style={{ padding: '20px' }}>
      <h1>API Gateway Example: Posts</h1>
      {posts.map(post => (
        <Post key={post.id} title={post.title} body={post.body} />
      ))}
    </div>
  );
};

const rootElement = document.getElementById('root');
ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  rootElement
);