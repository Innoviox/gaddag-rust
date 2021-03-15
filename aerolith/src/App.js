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

  updateItem = (i, e) => {
      this.setState(state => {
          const inputs = state.inputs.map((item, j) => {
              if (j === i) {
                  return e.target.value;
              } else {
                  return item;
              }
          });

          return {
              inputs,
          };
      });
  }

  render = () =>
    <div>
        <div id="inputs">
            {this.state.inputs.map((item, i) => (
                <input key={i} ref={i} type="text" onChange={e => this.updateItem(i, e)} />
            ))}
        </div>
    </div>;
}

export default App;
