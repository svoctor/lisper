import { React, Component } from "react";

import styles from '../styles/Home.module.css'

class ToggleButton extends Component {
    render() {
        return (
            <button type="button" className={styles.themeButton} onClick={this.props.onClick}>
                <svg width="16" height="16" viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="100" cy="100" r="100" fill={this.props.enabled ? "#FFD34B" : "#F2F2F2"}/>
                </svg>
            </button>
            );
    }
}

export default ToggleButton;