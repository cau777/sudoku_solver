import React, {useState} from 'react';
import './App.css';
import init, {add, solve} from "wasm";
import {SudokuController} from "./SudokuController";

const example = `
6 3 _ _ _ _ _ 8 1
_ 2 _ _ _ 3 _ _ _
_ _ _ _ 1 7 4 3 _
_ 9 _ 4 _ _ 5 7 _
_ _ _ 7 6 2 _ _ _
_ 8 _ _ _ _ 6 _ _
_ 6 _ _ 2 _ _ _ _
3 _ 9 _ _ _ _ 6 _
_ _ _ _ _ _ _ _ 9`;

function App() {
    const [ans, setAns] = useState<any>();
    // useEffect(() => {
    //     init().then(() => {
    //         setAns(add(1, 6));
    //     })
    // }, []);
    
    init().then();
    console.log(ans)
    
    return (
        <div className="App">
            <SudokuController></SudokuController>
            {ans}
            <button onClick={() => setAns(add(3, 0))}>Test add</button>
            <button onClick={() => setAns(solve(example, 3, 0))}>Test colve</button>
        </div>
    );
}

export default App;
