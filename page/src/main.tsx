import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import {init} from 'ns9_1'

init()
ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App/>
    </React.StrictMode>,
)
