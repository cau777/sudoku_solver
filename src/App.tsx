import React, {useEffect, useState} from 'react';
import logo from './logo.svg';
import './App.css';
import init, {add} from "wasm";

function App() {
    const [ans, setAns] = useState(0);
    useEffect(() => {
        init().then(() => {setAns(add(1, 6));})
    }, []);
    
    return (
        <div className="App">
            <header className="App-header">
                {ans}
            </header>
        </div>
    );
}

export default App;
