import { Inter } from '@next/font/google'
import styles from '../styles/Home.module.css'
import { invoke } from '@tauri-apps/api/tauri'
import { useState, useEffect } from 'react'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const [greeting, setGreeting] = useState<string>("")

  useEffect(() => {
    invoke<string>('greet', { name: 'World' })
      .then(setGreeting)
      .catch((error: any) => console.error(error));
  }, []);

  return (
    <>
      <main className={styles.main}>
        <h1>Example Tauri App</h1>
        <p>{greeting}</p>
      </main>
    </>
  )
}
