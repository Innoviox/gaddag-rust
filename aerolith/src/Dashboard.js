import './Dashboard.css';
import {Component} from "react/cjs/react.production.min";
import React from "react";


class Dashboard extends Component {
    constructor(props) {
        super(props);

        this.state = {
        };
    }

    render = () =>
        <html className="has-navbar-fixed-top">
        <head>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.2/css/bulma.min.css" />
        </head>
        <body>
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
        </body>
        </html>;
}

export default Dashboard;
