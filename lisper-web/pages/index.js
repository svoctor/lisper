import { useState, useEffect } from 'react'
import Head from 'next/head'
import LisperEditor from '../components/LisperEditor'
import LisperOutput from '../components/LisperOutput'
import styles from '../styles/Home.module.css'
import { useTheme } from 'next-themes'
import ToggleButton from '../components/ToggleButton';


const codeExample = "(def a 10)\n(def b 11)\n(+ a b)";

// Load wasm-pack's browser-native ES module without Webpack transforming its WASM glue.
let lisperPromise;
const loadLisper = () => {
  if (!lisperPromise) {
    lisperPromise = import(/* webpackIgnore: true */ '/wasm/lisper_wasm.js')
      .then(async (module) => {
        await module.default({ module_or_path: '/wasm/lisper_wasm_bg.wasm' });
        return module;
      })
      .catch((error) => {
        lisperPromise = undefined;
        throw error;
      });
  }
  return lisperPromise;
};

const Home = () => {
  const [mounted, setMounted] = useState(false)
  let [code, updateCode] = useState({ source: "" });
  let [output, updateOutput] = useState("");
    
  const { theme, setTheme, resolvedTheme } = useTheme();
  
  useEffect(() => {
    // Initialize the interpreter once, not on every render.
    evaluate({ source: codeExample });
    setMounted(true);
  }, []);

  if (!mounted) return null

  async function evaluate(exp) {
    // Update code prop to reflect in the editor
    updateCode(exp);
    const interpreter = await loadLisper();
    // Evaluate the latest editor value only; an earlier async load must not overwrite it.
    updateOutput(interpreter.run(exp.source));
  }

  function toggleTheme() {
    setTheme(resolvedTheme == 'light' ? 'dark' : 'light');
  }
  
  return (
    <div className={styles.container}>
      <Head>
        <title>Lisper</title>
        <meta name="theme-color" content={resolvedTheme === 'dark' ? '#252526' : '#ffffff'} />
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
            <div className={styles.info}>
              <p>A project for learning and exploration, Lisper is an interactive Lisp interpreter written in Rust and compiled to WebAssembly.</p>
              <p>Check out the <a href="https://github.com/svoctor/lisper/">github repo</a> for more info and details on how to use it.</p>
            </div>
          </div>
          <div className={styles.editor}>
            <LisperEditor code={ code } onUpdate={evaluate} theme={resolvedTheme} />
          </div>

          <div className={styles.output}>
            <LisperOutput content={ output } />
          </div>
        </div>
      </main>
      <footer className={styles.footer}>
        <ToggleButton onClick={toggleTheme} enabled={resolvedTheme == 'light'}/>
      </footer>
    </div>
  )
}

export default Home;