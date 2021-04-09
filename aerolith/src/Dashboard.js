import './Dashboard.css';
import {Component} from "react/cjs/react.production.min";
import React from "react";


class Dashboard extends Component {
    constructor(props) {
        super(props);

        this.state = {
        };
    }

    componentDidMount() {
        window.addEventListener('load', this.handleLoad);
        document.getElementsByTagName("html")[0].addEventListener('click', e => {
            document.getElementsByTagName("html")[0].classList.toggle("is-clipped");
            document.getElementById("playmodal").classList.toggle("is-active");
        });
    }

    componentWillUnmount() {
        window.removeEventListener('load', this.handleLoad);
    }

    handleLoad() {
        document.getElementsByTagName("html")[0].classList.add("has-navbar-fixed-top");
    }

    render = () =>
        <div className="blocK">
        <nav className="navbar is-fixed-top is-dark" role="navigation" aria-label="main navigation">
            <div className="navbar-brand">
                <h1 className="navbar-item">Myrolith</h1>
            </div>

            <div id="navbarBasicExample" className="navbar-menu">
                <div className="navbar-end">
                    <div className="navbar-item">
                        <div className="buttons">
                            <a className="button is-primary">
                                <strong>Sign up</strong>
                            </a>
                            <a className="button is-light">
                                Log in
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
        <br />
        <br />
        <div className="block" id="main">
            <button className="button is-primary" id="playbtn">Play</button>
        </div>
        <div className="modal" id="playmodal">
            <div className="modal-background"></div>
            <div className="modal-card">
                <header className="modal-card-head">
                    <p className="modal-card-title">Modal title</p>
                    <button className="delete" aria-label="close"></button>
                </header>
                <section className="modal-card-body">
                    Content
                </section>
                <footer className="modal-card-foot">
                    <button className="button is-success">Save changes</button>
                    <button className="button">Cancel</button>
                </footer>
            </div>
        </div>
        </div>;
}

export default Dashboard;
