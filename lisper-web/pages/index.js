import {useState} from 'react'
import Head from 'next/head'
import LisperEditor from '../components/LisperEditor'
import LisperOutput from '../components/LisperOutput'
import styles from '../styles/Home.module.css'
import { useTheme } from 'next-themes'
import ToggleButton from '../components/ToggleButton';

const codeExample = `(+ 1 1)`;

const loadLisper = () => import('lisper-wasm');


const Home = () => {
  let [code, updateCode] = useState({ source: codeExample });
  let [output, updateOutput] = useState("waiting");
  var lisper = null;
  
  const darkTheme = require('prismjs/themes/prism-dark.css');
  
  const { theme, setTheme } = useTheme();
  
  async function evaluate(exp) {
    // Update code prop to reflect in the editor
    updateCode(exp);

    if (lisper == null){
      lisper = await loadLisper();
    }
    // evaluate expression and set result to output
    updateOutput(lisper.run(exp.source));
  }

  function toggleTheme() {
    if (theme == 'light') {
      setTheme('dark');
    } else {
      setTheme('light');
    }
  }
  
  return (
    <div className={styles.container}>
      <Head>
        <title>Lisper</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        <div className={styles.codeArea}>
          <div>
            <div className={styles.title}>Lisper</div>
          </div>
          <div>
            <div className={styles.info}>Lisper is a project to learn and explore Rust and  WebAssembly. It is an interactive Lisp interpreter, check out the <a href="https://github.com/svoctor/lisper/">github repo</a> for more info and details on how to use it.</div>
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