import { useState, useEffect } from 'react'
import Head from 'next/head'
import LisperEditor from '../components/LisperEditor'
import LisperOutput from '../components/LisperOutput'
import styles from '../styles/Home.module.css'
import { useTheme } from 'next-themes'
import ToggleButton from '../components/ToggleButton';

const codeExample = "(def a 10)\n(def b 11)\n(+ a b)";

const loadLisper = () => import('lisper-wasm');
let lisper = null;

const Home = () => {
  let [code, updateCode] = useState({ source: "" });
  let [output, updateOutput] = useState("");
  
  const darkTheme = require('prismjs/themes/prism-dark.css');
  
  const { theme, setTheme } = useTheme();
  
  useEffect(() => {
    // On first load, refresh the editor with sample code
    // which also loads the lisper module
    if(lisper == null) {
      evaluate({ source: codeExample });
    }
  });
  
  async function evaluate(exp) {
    // Update code prop to reflect in the editor
    updateCode(exp);
    if (lisper == null) {
      lisper = await loadLisper();
    }
    // evaluate expression and set result to output
    updateOutput(lisper.run(exp.source));
  }

  function toggleTheme() {
    setTheme(theme == 'light' ? 'dark' : 'light');
  }
  
  return (
    <div className={styles.container}>
      <Head>
        <title>Lisper</title>
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <link rel="manifest" href="/site.webmanifest"/>
      </Head>
      <main className={styles.main}>
        <div className={styles.codeArea}>
          <div>
            <div className={styles.title}>Lisper</div>
          </div>
          <div>
            <div className={styles.info}>A project for learning and exploration, Lisper is an interactive Lisp interpreter written in Rust and compiled to WebAssembly.<br />Check out the <a href="https://github.com/svoctor/lisper/">github repo</a> for more info and details on how to use it.</div>
          </div>
          <div className={styles.editor}>
            <LisperEditor code={ code } onUpdate={evaluate} theme={theme} />
          </div>

          <div className={styles.output}>
            <LisperOutput content={ output } />
          </div>
        </div>
      </main>
      <footer className={styles.footer}>
        <ToggleButton onClick={toggleTheme} enabled={theme == 'light'}/>
      </footer>
    </div>
  )
}

export default Home;