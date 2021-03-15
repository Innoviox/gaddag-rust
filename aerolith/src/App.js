import logo from './logo.svg';
import './App.css';
import {Component} from "react/cjs/react.production.min";
import React from "react";

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
        inputs: ['', '', '', '', '', '', ''],
        words: [],
        wordI: 0,
        currGuess: 0
    };

    this.updateItem = this.updateItem.bind(this);
    this.setWords = this.setWords.bind(this);

    fetch(process.env.PUBLIC_URL + '/files/sevens1000-out.txt')
        .then(r => r.text())
        .then(this.setWords);
  }

  input = i => document.querySelector(`input[name=input-${i}]`);
  inputs = () => [0, 1, 2, 3, 4, 5, 6].map(this.input);
  currWord = () => this.state.words[this.state.wordI];
  clearInputs = () => this.inputs().map(i => i.value = "");

  updateItem = (i, e) => {
      this.setState({ inputs: this.state.inputs.map((item, j) => j === i ? e.target.value : item) });

      this.input(i).value = this.input(i).value.toUpperCase();

      const next = this.input(i + 1);
      if (next === null) {
          let guess = this.inputs().map(i => i.value).join("");
          if (this.currWord().w.includes(guess)) {
              this.setState( { currGuess: this.state.currGuess + 1}, () => this.flickerClass('green', 500, () => {
                  this.clearInputs();
                  if (this.state.currGuess === this.currWord().w.length) {
                      this.setState( { wordI: this.state.wordI + 1 }, this.writeAnagram);
                  }
              }))
          } else {
              this.flickerClass('red', 500, this.clearInputs)
          }
      } else {
          next.focus();
      }
  }

  flickerClass = (cls, time, callback) => {
      document.activeElement.blur();
      this.inputs().map(i => {
          i.classList.toggle(cls);
          setTimeout(() => i.classList.toggle(cls), time);
      });
      setTimeout(callback, time);
  }

  setWords = (text) => {
      this.setState({ words: text.split("\n").map(i => {
              let [anagram, words] = i.split(",");
              words = words.split(" ");
              return {'a': anagram, 'w': words};
          })}, this.writeAnagram);
  }

  writeAnagram = () => {
      document.getElementById("test").innerHTML = this.currWord().a;
      this.input(0).focus();
  }

  render = () =>
    <div id="test-div">
        <div id="inputs">
            {this.state.inputs.map((item, i) => (
                <input maxLength={1} key={i} name={`input-${i}`} className="test-input" type="text" onChange={e => this.updateItem(i, e)} />
            ))}
        </div>
        <span id="test" />
    </div>;
}

export default App;
