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
    }

    componentWillUnmount() {
        window.removeEventListener('load', this.handleLoad);
    }

    handleLoad() {
        document.getElementsByTagName("html")[0].classList.add("has-navbar-fixed-top");
    }

    render = () =>
        <div>
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
        <div>

        </div>
        </div>;
}

export default Dashboard;
