import { React, useState } from "react";
import Editor from 'react-simple-code-editor';
import { highlight, languages } from 'prismjs/components/prism-core';
import 'prismjs/components/prism-lisp';

import styles from '../styles/Editor.module.css'

const LisperEditor = ({...props}) => {

    return (
        <Editor
          value={props.code.source}
          onValueChange={code => props.onUpdate({ source: code })}
          highlight={code => highlight(code, languages.lisp)}
          padding={10}
          style={{
            fontFamily: '"Fira code", "Fira Mono", monospace',
            fontSize: 12,
          }}
        />
    )
}

export default LisperEditor;
