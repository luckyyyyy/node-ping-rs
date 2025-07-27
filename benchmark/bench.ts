import { Bench } from 'tinybench'
import { ping } from '../index.js'
// 子进程
import { spawn } from 'child_process'

const b = new Bench()

// 使用子进程执行系统 ping 命令
const processPing = (host: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const pingProcess = spawn('ping', ['-c', '1', host])

    pingProcess.on('close', (code) => {
      if (code === 0) {
        resolve()
      } else {
        reject(new Error(`ping process exited with code ${code}`))
      }
    })

    pingProcess.on('error', (error) => {
      reject(error)
    })
  })
}

b.add('ping (Rust NAPI)', async () => {
  await ping('10.0.0.1')
})

b.add('ping (System Process)', async () => {
  await processPing('10.0.0.1')
})


await b.run()

console.table(b.table())
