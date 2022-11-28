import React, {useEffect, useState} from 'react';
import './App.css';
import {SudokuController} from "./SudokuController";
import {useTranslation} from "react-i18next";

type State = {
    log: string;
}

function App() {
    let [state, setState] = useState<State>({log: ""});
    let {t} = useTranslation();
    
    useEffect(() => {
        document.title = t("title");
    }, [t])
    
    return (
        <div className="App">
            <div className={"main-container"}>
                <div className={"main-card"}>
                    <h1>{t("title")}</h1>
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
