import { React, useState } from "react";
import Editor from 'react-simple-code-editor';

var unified = require('unified')
var rehypeStringify = require('rehype-stringify')
var processor = unified().use(rehypeStringify)
var refractor = require('refractor/core')
refractor.register(require('refractor/lang/lisp'))

import styles from '../styles/Editor.module.css'

const LisperEditor = ({...props}) => {

    return (
      <div>
        <Editor
          value={props.code.source}
          onValueChange={code => props.onUpdate({ source: code })}
          highlight={code => {
            let tree = refractor.highlight(code, 'lisp');
            var html = processor.stringify({type: 'root', children: tree}).toString();
            return html;
          }}
          padding={10}
          className={styles.codeEditor}
        />
      </div>
    )
}

export default LisperEditor;
