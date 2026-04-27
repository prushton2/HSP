import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div className="title">
        <h1>HSP</h1>
      </div>

      <div className="body">
        <div className="titleButton" onClick={() => window.location.href="/admin"}>Admin</div>
        <div className="titleButton" onClick={() => window.location.href="/nursing"}>Nursing Numbers</div>
        <div className="titleButton" onClick={() => window.location.href="/activities"}>Activities</div>
        <div className="titleButton" onClick={() => window.location.href="/search"}>Search</div>
      </div>

    </>
  )
}

export default App
