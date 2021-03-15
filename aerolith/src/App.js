import logo from './logo.svg';
import './App.css';
import {Component} from "react/cjs/react.production.min";
import React from "react";

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
        inputs: ['', '', '', '', '', '', '']
    };

    this.updateItem = this.updateItem.bind(this);

    // fetch(process.env.PUBLIC_URL + '/files/sevens1000-out.txt')
    //     .then(r => r.text())
    //     .then(text => {
    //         console.log('text decoded:', text);
    //     });
  }

  input = i => document.querySelector(`input[name=input-${i}]`);

  updateItem = (i, e) => {
      this.setState({ inputs: this.state.inputs.map((item, j) => j === i ? e.target.value : item) });

      this.input(i).value = this.input(i).value.toUpperCase();

      const next = this.input(i + 1);
      if (next === null) {

      } else {
          next.focus();
      }
  }

  render = () =>
    <div>
        <div id="inputs">
            {this.state.inputs.map((item, i) => (
                <input maxLength={1} key={i} name={`input-${i}`} className="test-input" type="text" onChange={e => this.updateItem(i, e)} />
            ))}
        </div>
    </div>;
}

export default App;
