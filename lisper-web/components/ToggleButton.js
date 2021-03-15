import { React, Component } from "react";

import styles from '../styles/Home.module.css'

export class ToggleButton extends Component {

    constructor(props) {
        super(props);
        this.state = {
            on: props.enabled,
            color: props.enabled  ? "#FFD34B" : "#F2F2F2" ,
        };
        this.toggle = this.toggle.bind(this);
    }
    
    toggle() {
        this.state.on = !this.state.on;
        this.state.color = this.state.on ? "#FFD34B" : "#F2F2F2",
        this.props.onClick();
    }
    
    render() {
       return (
        <button type="button" className={styles.themeButton} onClick={this.toggle}>
            <svg width="16" height="16" viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="100" cy="100" r="100" fill={this.state.color}/>
            </svg>
        </button>
        );
    }
}

export default ToggleButton;