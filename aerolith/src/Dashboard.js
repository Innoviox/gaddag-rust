import './Dashboard.css';
import {Component} from "react/cjs/react.production.min";
import React from "react";


class App extends Component {
    constructor(props) {
        super(props);

        this.state = {
            inputs: ['', '', '', '', '', '', ''],
            words: [],
            wordI: 0,
            currGuess: []
        };

        this.updateItem = this.updateItem.bind(this);
        this.setWords = this.setWords.bind(this);

        fetch(process.env.PUBLIC_URL + '/files/sevens1000-out.txt')
            .then(r => r.text())
            .then(this.setWords);
    }

    render = () =>
        <div id="test-div">
            <div id="inputs">
                {this.state.inputs.map((item, i) => (
                    <input maxLength={1} key={i} name={`input-${i}`} className="test-input" type="text" onChange={e => this.updateItem(i, e)} />
                ))}
            </div>
            <br />
            <span id="test" />
        </div>;
}

export default App;
