import { React, useState } from "react";

import styles from '../styles/Output.module.css'

const LisperOutput = ({...props}) => {

    return (
        <div className={styles.output}>
            { props.content }
        </div>
    )
}

export default LisperOutput;
