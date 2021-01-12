import {useState} from 'react'
import Head from 'next/head'
import LisperEditor from '../components/LisperEditor'
import LisperOutput from '../components/LisperOutput'
import styles from '../styles/Home.module.css'

const codeExample = `(+ 1 1)`;

const loadLisper = () => import('lisper-wasm');

const Home = () => {
  let [code, updateCode] = useState({ source: codeExample });
  let [output, updateOutput] = useState("waiting");
  var lisper = null;
  
  async function evaluate(exp) {
    // Update code prop to reflect in the editor
    updateCode(exp);

    if (lisper == null){
      lisper = await loadLisper();
    }
    // evaluate expression and set result to output
    updateOutput(lisper.run(exp.source));
  }

  return (
    <div className={styles.container}>
      <Head>
        <title>Lisper</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
    
      <main className={styles.main}>
        <div className={styles.editor}>
          <LisperEditor code={ code } onUpdate={ evaluate } />
        </div>
        <div className={styles.output}>
          <LisperOutput content={ output } />
        </div>
      </main>
    </div>
  )
}

export default Home;