import React, {useState} from 'react';
import './App.css';
import {SudokuController} from "./SudokuController";

type State = {
    log: string;
}

function App() {
    let [state, setState] = useState<State>({log: ""});
    
    return (
        <div className="App">
            <div className={"main-container"}>
                <div className={"main-card"}>
                    <h1>Sudoku Solver</h1>
                    <div style={{overflow: "auto"}}>
                        <SudokuController setLog={o => setState({...state, log: o})}></SudokuController>
                    </div>
                    <p>{state.log}</p>
                </div>
            </div>
        </div>
    );
}

export default App;
